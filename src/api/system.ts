/**
 * System API functions for interacting with the operating system and Tauri backend.
 *
 * This module provides interfaces for:
 * - System information retrieval
 * - Window management operations
 * - Secure file system operations with path validation
 * - Command execution with allowlist restrictions
 * - Utility functions for file handling
 *
 * All file operations are secured against path traversal attacks.
 */

import { invoke } from '@tauri-apps/api/core'
import type {
  SystemInfo,
  WindowInfo,
  DirectoryListing,
  FileInfo,
} from '../types/system'

// ==================== System Information ====================
/** Retrieves system information including platform, architecture, and version. */
export const getSystemInfo = async (): Promise<SystemInfo> => {
  return await invoke('get_system_info')
}

/** Gets the application's data directory path. */
export const getAppDataDir = async (): Promise<string> => {
  return await invoke('get_app_data_dir')
}

/** Gets the application's log directory path. */
export const getAppLogDir = async (): Promise<string> => {
  return await invoke('get_app_log_dir')
}

// ==================== Notifications ====================

/** Sends a desktop notification with the specified title and body. */
export const sendNotification = async (
  title: string,
  body: string
): Promise<string> => {
  return await invoke('send_notification', { title, body })
}

// ==================== Window Management ====================

/** Retrieves information about the current window state. */
export const getWindowInfo = async (): Promise<WindowInfo> => {
  return await invoke('get_window_info')
}

/** Toggles the window between maximized and restored states. */
export const toggleWindowMaximize = async (): Promise<string> => {
  return await invoke('toggle_window_maximize')
}

/** Minimizes the application window. */
export const minimizeWindow = async (): Promise<string> => {
  return await invoke('minimize_window')
}

/** Centers the window on the screen. */
export const centerWindow = async (): Promise<string> => {
  return await invoke('center_window')
}

/** Sets the window title to the specified text. */
export const setWindowTitle = async (title: string): Promise<string> => {
  return await invoke('set_window_title', { title })
}

/** Creates a new application window with the specified label and URL. */
export const createNewWindow = async (
  label: string,
  url: string
): Promise<string> => {
  return await invoke('create_new_window', { label, url })
}

// ==================== Command Execution ====================

/** Executes a system command from the allowlist with specified arguments. */
export const executeCommand = async (
  command: string,
  args: string[] = []
): Promise<string> => {
  return await invoke('execute_command', { command, args })
}

// ==================== File System Operations ====================

/** Reads the contents of a text file at the specified path. */
export const readTextFile = async (path: string): Promise<string> => {
  return await invoke('read_text_file', { path })
}

/** Writes text content to a file at the specified path. */
export const writeTextFile = async (
  path: string,
  content: string
): Promise<string> => {
  return await invoke('write_text_file', { path, content })
}

/** Appends text content to an existing file. */
export const appendTextFile = async (
  path: string,
  content: string
): Promise<string> => {
  return await invoke('append_text_file', { path, content })
}

/** Deletes a file at the specified path. */
export const deleteFile = async (path: string): Promise<string> => {
  return await invoke('delete_file', { path })
}

/** Creates a directory at the specified path. */
export const createDirectory = async (path: string): Promise<string> => {
  return await invoke('create_directory', { path })
}

/** Lists all files and directories at the specified path. */
export const listDirectory = async (
  path: string
): Promise<DirectoryListing> => {
  return await invoke('list_directory', { path })
}

/** Checks if a file or directory exists at the specified path. */
export const fileExists = async (path: string): Promise<boolean> => {
  return await invoke('file_exists', { path })
}

/** Gets detailed information about a file or directory. */
export const getFileInfo = async (path: string): Promise<FileInfo> => {
  return await invoke('get_file_info', { path })
}

/** Copies a file from source to destination path. */
export const copyFile = async (
  source: string,
  destination: string
): Promise<string> => {
  return await invoke('copy_file', { source, destination })
}

/** Moves a file from source to destination path. */
export const moveFile = async (
  source: string,
  destination: string
): Promise<string> => {
  return await invoke('move_file', { source, destination })
}

// ==================== Utility Functions ====================

/** Formats a file size in bytes to a human-readable string. */
export const formatFileSize = (bytes: number): string => {
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  if (bytes === 0) return '0 Bytes'
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + ' ' + sizes[i]
}

/** Extracts the file extension from a filename. */
export const getFileExtension = (filename: string): string => {
  return filename.slice(((filename.lastIndexOf('.') - 1) >>> 0) + 2)
}

/** Extracts the filename from a full file path. */
export const getFileName = (path: string): string => {
  return path.split(/[\\/]/).pop() || ''
}

/** Gets the parent directory path from a file path. */
export const getParentDirectory = (path: string): string => {
  const parts = path.split(/[\\/]/)
  parts.pop()
  return parts.join('/')
}
