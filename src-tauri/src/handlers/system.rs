use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Window};
use tauri_plugin_notification::{NotificationExt, PermissionState};

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub version: String,
    pub hostname: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WindowInfo {
    pub label: String,
    pub title: String,
    pub is_maximized: bool,
    pub is_minimized: bool,
    pub is_visible: bool,
    pub is_focused: bool,
    pub position: (i32, i32),
    pub size: (u32, u32),
}
const ALLOWED_COMMANDS: &[&str] = &[
    "npm", "npx", "pnpm", "yarn", "bun", "cargo", "rustup", "tauri", "node", "deno", "python",
    "pip", "pip3", "echo",
];

const MAX_ARGS: usize = 20;
const MAX_ARG_LEN: usize = 2048;

#[tauri::command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        version: "Unknown".to_string(), // Would use OS-specific calls in production
        hostname: hostname::get()
            .map_err(|e| format!("Failed to get hostname: {}", e))?
            .to_string_lossy()
            .to_string(),
    })
}

#[tauri::command]
pub async fn send_notification(
    app: AppHandle,
    title: String,
    body: String,
) -> Result<String, String> {
    let title = title.trim();
    let body = body.trim();

    if title.is_empty() && body.is_empty() {
        return Err("Notification title or body must be provided".to_string());
    }

    let notification = app.notification();

    match notification.permission_state() {
        Ok(PermissionState::Denied) => {
            return Err("Notification permission denied by the user".to_string());
        }
        Ok(PermissionState::Prompt | PermissionState::PromptWithRationale) => {
            match notification.request_permission() {
                Ok(PermissionState::Denied) => {
                    return Err("Notification permission denied by the user".to_string());
                }
                Ok(_) => {}
                Err(err) => {
                    return Err(format!(
                        "Failed to request notification permission: {}",
                        err
                    ));
                }
            }
        }
        Err(err) => {
            return Err(format!(
                "Failed to read notification permission state: {}",
                err
            ));
        }
        _ => {}
    }

    let mut builder = notification.builder();

    if !title.is_empty() {
        builder = builder.title(title);
    }

    if !body.is_empty() {
        builder = builder.body(body);
    }

    builder
        .show()
        .map_err(|e| format!("Failed to display notification: {}", e))?;

    Ok("Notification dispatched".to_string())
}

#[tauri::command]
pub async fn get_window_info(window: Window) -> Result<WindowInfo, String> {
    let label = window.label().to_string();
    let title = window.title().map_err(|e| e.to_string())?;
    let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
    let is_minimized = window.is_minimized().map_err(|e| e.to_string())?;
    let is_visible = window.is_visible().map_err(|e| e.to_string())?;
    let is_focused = window.is_focused().map_err(|e| e.to_string())?;

    let position = window.outer_position().map_err(|e| e.to_string())?;
    let size = window.outer_size().map_err(|e| e.to_string())?;

    Ok(WindowInfo {
        label,
        title,
        is_maximized,
        is_minimized,
        is_visible,
        is_focused,
        position: (position.x, position.y),
        size: (size.width, size.height),
    })
}

#[tauri::command]
pub async fn toggle_window_maximize(window: Window) -> Result<String, String> {
    if window.is_maximized().map_err(|e| e.to_string())? {
        window.unmaximize().map_err(|e| e.to_string())?;
        Ok("Window unmaximized".to_string())
    } else {
        window.maximize().map_err(|e| e.to_string())?;
        Ok("Window maximized".to_string())
    }
}

#[tauri::command]
pub async fn minimize_window(window: Window) -> Result<String, String> {
    window.minimize().map_err(|e| e.to_string())?;
    Ok("Window minimized".to_string())
}

#[tauri::command]
pub async fn center_window(window: Window) -> Result<String, String> {
    window.center().map_err(|e| e.to_string())?;
    Ok("Window centered".to_string())
}

#[tauri::command]
pub async fn set_window_title(window: Window, title: String) -> Result<String, String> {
    window.set_title(&title).map_err(|e| e.to_string())?;
    Ok(format!("Window title set to: {}", title))
}

#[tauri::command]
pub async fn create_new_window(
    app: AppHandle,
    label: String,
    url: String,
) -> Result<String, String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};

    let webview_url = if url.starts_with("http") {
        WebviewUrl::External(url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
    } else {
        WebviewUrl::App(url.into())
    };

    WebviewWindowBuilder::new(&app, &label, webview_url)
        .title("New Window")
        .inner_size(800.0, 600.0)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(format!("New window '{}' created", label))
}

#[tauri::command]
pub async fn execute_command(command: String, args: Vec<String>) -> Result<String, String> {
    use tokio::process::Command;

    let command = command.trim();
    if command.is_empty() {
        return Err("Command cannot be empty".to_string());
    }

    if command
        .chars()
        .any(|ch| ch.is_whitespace() || ch == '/' || ch == '\\')
    {
        return Err("Command contains invalid characters".to_string());
    }

    if !ALLOWED_COMMANDS
        .iter()
        .any(|allowed| allowed.eq_ignore_ascii_case(command))
    {
        return Err(format!(
            "Command '{}' is not permitted. Update the allow list to enable it.",
            command
        ));
    }

    if args.len() > MAX_ARGS {
        return Err(format!(
            "Too many arguments supplied. Maximum allowed is {}.",
            MAX_ARGS
        ));
    }

    if let Some(bad_arg) = args.iter().find(|arg| arg.len() > MAX_ARG_LEN) {
        return Err(format!(
            "Argument '{}' exceeds the maximum length of {} characters.",
            bad_arg, MAX_ARG_LEN
        ));
    }

    if let Some(bad_arg) = args.iter().find(|arg| arg.chars().any(|c| c == '\0')) {
        return Err(format!(
            "Argument '{}' contains invalid characters.",
            bad_arg
        ));
    }

    let resolved_command = ALLOWED_COMMANDS
        .iter()
        .find(|allowed| allowed.eq_ignore_ascii_case(command))
        .copied()
        .unwrap_or(command);

    let output = Command::new(resolved_command)
        .args(&args)
        .output()
        .await
        .map_err(|e| format!("Failed to execute command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if stdout.is_empty() {
            Ok("Command executed successfully.".to_string())
        } else {
            Ok(stdout)
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let code = output
            .status
            .code()
            .map(|c| c.to_string())
            .unwrap_or_else(|| "terminated by signal".to_string());

        Err(format!("Command exited with {code}: {stderr}"))
    }
}

#[tauri::command]
pub async fn get_app_data_dir(app: AppHandle) -> Result<String, String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;

    Ok(app_data_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn get_app_log_dir(app: AppHandle) -> Result<String, String> {
    let app_log_dir = app.path().app_log_dir().map_err(|e| e.to_string())?;

    Ok(app_log_dir.to_string_lossy().to_string())
}
