use chrono::{DateTime, Utc};
use directories::ProjectDirs;
use dunce::canonicalize;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::time::SystemTime;

const ROOT_ENV_OVERRIDE: &str = "TAURI_FS_ROOT";
const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "tavuc";
const APP_NAME: &str = "tavuc-boilerplate";

#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub is_file: bool,
    pub modified: Option<String>,
    pub created: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryListing {
    pub path: String,
    pub entries: Vec<FileInfo>,
}

struct FsContext {
    root: PathBuf,
    path: PathBuf,
}

impl FsContext {
    fn relative_display(&self) -> String {
        self.path
            .strip_prefix(&self.root)
            .ok()
            .map(relative_path_to_string)
            .unwrap_or_else(|| relative_path_to_string(Path::new(".")))
    }
}

#[tauri::command]
pub async fn read_text_file(path: String) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let context = resolve_existing_path(&path)?;

    if !context.path.is_file() {
        return Err(format!(
            "Path '{}' is not a file",
            context.relative_display()
        ));
    }

    fs::read_to_string(&context.path).map_err(|e| {
        format!(
            "Failed to read file '{}': {}",
            context.relative_display(),
            e
        )
    })
}

#[tauri::command]
pub async fn write_text_file(path: String, content: String) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let context = resolve_relative_path(&path)?;

    if context.path == context.root {
        return Err("Refusing to overwrite the filesystem root".to_string());
    }

    if let Some(parent) = context.path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directory for '{}': {}",
                context.relative_display(),
                e
            )
        })?;
    }

    fs::write(&context.path, content).map_err(|e| {
        format!(
            "Failed to write file '{}': {}",
            context.relative_display(),
            e
        )
    })?;

    Ok(format!(
        "File '{}' written successfully",
        context.relative_display()
    ))
}

#[tauri::command]
pub async fn append_text_file(path: String, content: String) -> Result<String, String> {
    use std::fs::OpenOptions;
    use std::io::Write;

    if path.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let context = resolve_relative_path(&path)?;

    if context.path == context.root {
        return Err("Refusing to modify the filesystem root".to_string());
    }

    if let Some(parent) = context.path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create parent directory for '{}': {}",
                context.relative_display(),
                e
            )
        })?;
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&context.path)
        .map_err(|e| {
            format!(
                "Failed to open file '{}': {}",
                context.relative_display(),
                e
            )
        })?;

    file.write_all(content.as_bytes()).map_err(|e| {
        format!(
            "Failed to append to file '{}': {}",
            context.relative_display(),
            e
        )
    })?;

    Ok(format!(
        "Content appended to file '{}'",
        context.relative_display()
    ))
}

#[tauri::command]
pub async fn delete_file(path: String) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let context = resolve_existing_path(&path)?;

    if context.path == context.root {
        return Err("Refusing to delete the filesystem root".to_string());
    }

    if context.path.is_file() {
        fs::remove_file(&context.path).map_err(|e| {
            format!(
                "Failed to delete file '{}': {}",
                context.relative_display(),
                e
            )
        })?;

        Ok(format!(
            "File '{}' deleted successfully",
            context.relative_display()
        ))
    } else if context.path.is_dir() {
        fs::remove_dir_all(&context.path).map_err(|e| {
            format!(
                "Failed to delete directory '{}': {}",
                context.relative_display(),
                e
            )
        })?;

        Ok(format!(
            "Directory '{}' deleted successfully",
            context.relative_display()
        ))
    } else {
        Err(format!(
            "Path '{}' does not exist",
            context.relative_display()
        ))
    }
}

#[tauri::command]
pub async fn create_directory(path: String) -> Result<String, String> {
    if path.trim().is_empty() {
        return Err("Path cannot be empty".to_string());
    }

    let context = resolve_relative_path(&path)?;

    if context.path == context.root {
        return Err("The filesystem root already exists".to_string());
    }

    fs::create_dir_all(&context.path).map_err(|e| {
        format!(
            "Failed to create directory '{}': {}",
            context.relative_display(),
            e
        )
    })?;

    Ok(format!(
        "Directory '{}' created successfully",
        context.relative_display()
    ))
}

#[tauri::command]
pub async fn list_directory(path: String) -> Result<DirectoryListing, String> {
    let context = resolve_relative_path(&path)?;

    if !context.path.exists() {
        return Err(format!(
            "Path '{}' does not exist",
            context.relative_display()
        ));
    }

    if !context.path.is_dir() {
        return Err(format!(
            "Path '{}' is not a directory",
            context.relative_display()
        ));
    }

    let entries = fs::read_dir(&context.path).map_err(|e| {
        format!(
            "Failed to read directory '{}': {}",
            context.relative_display(),
            e
        )
    })?;

    let mut file_infos = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let entry_path = entry.path();
        let metadata = entry
            .metadata()
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        file_infos.push(build_file_info(&entry_path, metadata, &context.root));
    }

    file_infos.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(DirectoryListing {
        path: context.relative_display(),
        entries: file_infos,
    })
}

#[tauri::command]
pub async fn file_exists(path: String) -> Result<bool, String> {
    let context = resolve_relative_path(&path)?;
    Ok(context.path.exists())
}

#[tauri::command]
pub async fn get_file_info(path: String) -> Result<FileInfo, String> {
    let context = resolve_existing_path(&path)?;
    let metadata = context.path.metadata().map_err(|e| {
        format!(
            "Failed to read metadata for '{}': {}",
            context.relative_display(),
            e
        )
    })?;

    Ok(build_file_info(&context.path, metadata, &context.root))
}

#[tauri::command]
pub async fn copy_file(source: String, destination: String) -> Result<String, String> {
    if source.trim().is_empty() || destination.trim().is_empty() {
        return Err("Source and destination paths cannot be empty".to_string());
    }

    let source_context = resolve_existing_path(&source)?;

    if source_context.path == source_context.root {
        return Err("Copying the filesystem root is not permitted".to_string());
    }

    if !source_context.path.exists() {
        return Err(format!(
            "Source path '{}' does not exist",
            source_context.relative_display()
        ));
    }

    let destination_context = resolve_relative_path(&destination)?;

    if destination_context.path == destination_context.root {
        return Err("Destination path cannot be the filesystem root".to_string());
    }

    if let Some(parent) = destination_context.path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create destination directory '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    fs::copy(&source_context.path, &destination_context.path).map_err(|e| {
        format!(
            "Failed to copy '{}' to '{}': {}",
            source_context.relative_display(),
            destination_context.relative_display(),
            e
        )
    })?;

    Ok(format!(
        "File copied from '{}' to '{}'",
        source_context.relative_display(),
        destination_context.relative_display()
    ))
}

#[tauri::command]
pub async fn move_file(source: String, destination: String) -> Result<String, String> {
    if source.trim().is_empty() || destination.trim().is_empty() {
        return Err("Source and destination paths cannot be empty".to_string());
    }

    let source_context = resolve_existing_path(&source)?;

    if source_context.path == source_context.root {
        return Err("Moving the filesystem root is not permitted".to_string());
    }

    let destination_context = resolve_relative_path(&destination)?;

    if destination_context.path == destination_context.root {
        return Err("Destination path cannot be the filesystem root".to_string());
    }

    if let Some(parent) = destination_context.path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "Failed to create destination directory '{}': {}",
                parent.display(),
                e
            )
        })?;
    }

    fs::rename(&source_context.path, &destination_context.path).map_err(|e| {
        format!(
            "Failed to move '{}' to '{}': {}",
            source_context.relative_display(),
            destination_context.relative_display(),
            e
        )
    })?;

    Ok(format!(
        "File moved from '{}' to '{}'",
        source_context.relative_display(),
        destination_context.relative_display()
    ))
}

fn filesystem_root() -> Result<PathBuf, String> {
    let base = if let Ok(override_path) = env::var(ROOT_ENV_OVERRIDE) {
        PathBuf::from(override_path)
    } else if let Some(project_dirs) = ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME)
    {
        project_dirs.data_dir().to_path_buf()
    } else {
        env::current_dir().map_err(|e| format!("Failed to determine filesystem root: {}", e))?
    };

    fs::create_dir_all(&base).map_err(|e| {
        format!(
            "Failed to initialize filesystem root '{}': {}",
            base.display(),
            e
        )
    })?;

    canonicalize(&base).map_err(|e| {
        format!(
            "Failed to resolve filesystem root '{}': {}",
            base.display(),
            e
        )
    })
}

fn resolve_relative_path(raw: &str) -> Result<FsContext, String> {
    if raw.contains(' ') {
        return Err("Path contains invalid characters".to_string());
    }

    let candidate = PathBuf::from(raw.trim());

    if candidate.is_absolute() {
        return Err(
            "Absolute paths are not permitted. Provide a path relative to the application data directory.".to_string(),
        );
    }

    let root = filesystem_root()?;
    let mut normalized = root.clone();
    let mut depth = 0usize;

    for component in candidate.components() {
        match component {
            Component::Prefix(_) | Component::RootDir => {
                return Err(
                    "Absolute paths are not permitted. Provide a path relative to the application data directory.".to_string(),
                );
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if depth == 0 {
                    return Err(
                        "Path traversal outside the application directory is not permitted."
                            .to_string(),
                    );
                }
                normalized.pop();
                depth -= 1;
            }
            Component::Normal(segment) => {
                if segment.is_empty() {
                    continue;
                }
                normalized.push(segment);
                depth += 1;
            }
        }
    }

    Ok(FsContext {
        root,
        path: normalized,
    })
}

fn resolve_existing_path(raw: &str) -> Result<FsContext, String> {
    let context = resolve_relative_path(raw)?;

    if !context.path.exists() {
        return Err(format!(
            "Path '{}' does not exist",
            context.relative_display()
        ));
    }

    Ok(context)
}

fn build_file_info(path: &Path, metadata: fs::Metadata, root: &Path) -> FileInfo {
    let relative = path.strip_prefix(root).unwrap_or(path);
    let display_path = relative_path_to_string(relative);
    let name = relative
        .file_name()
        .map(|segment| segment.to_string_lossy().to_string())
        .filter(|name| !name.is_empty())
        .unwrap_or_else(|| display_path.clone());

    FileInfo {
        name,
        path: display_path,
        size: metadata.len(),
        is_dir: metadata.is_dir(),
        is_file: metadata.is_file(),
        modified: metadata.modified().ok().and_then(format_system_time),
        created: metadata.created().ok().and_then(format_system_time),
    }
}

fn relative_path_to_string(path: &Path) -> String {
    let value = path.to_string_lossy();
    if value.is_empty() {
        ".".to_string()
    } else {
        value.to_string()
    }
}

fn format_system_time(time: SystemTime) -> Option<String> {
    time.duration_since(std::time::UNIX_EPOCH)
        .ok()
        .and_then(|duration| DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0))
        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use once_cell::sync::Lazy;
    use std::sync::Mutex;
    use tauri::async_runtime::block_on;
    use tempfile::TempDir;

    static TEST_GUARD: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    fn with_temp_root<F: FnOnce(&Path) -> T, T>(callback: F) -> T {
        let _guard = TEST_GUARD.lock().expect("filesystem test mutex poisoned");
        let temp = TempDir::new().expect("failed to create temp dir");
        env::set_var(ROOT_ENV_OVERRIDE, temp.path());
        let result = callback(temp.path());
        env::remove_var(ROOT_ENV_OVERRIDE);
        result
    }

    #[test]
    fn prevents_path_traversal() {
        with_temp_root(|_| {
            let error = block_on(read_text_file("../evil.txt".into())).unwrap_err();
            assert!(error.contains("not permitted"));
        });
    }

    #[test]
    fn writes_and_reads_within_root() {
        with_temp_root(|_| {
            let write_message =
                block_on(write_text_file("nested/file.txt".into(), "hello".into())).unwrap();
            assert!(write_message.contains("nested"));

            let context = resolve_relative_path("nested/file.txt").expect("resolved path");
            assert_eq!(
                context.relative_display().replace("\\", "/"),
                "nested/file.txt"
            );

            let content = block_on(read_text_file("nested/file.txt".into())).unwrap();
            assert_eq!(content, "hello");
        });
    }

    #[test]
    fn rejects_root_deletion() {
        with_temp_root(|_| {
            let error = block_on(delete_file(".".into())).unwrap_err();
            assert!(error.contains("filesystem root"));
        });
    }
}
