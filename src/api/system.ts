import { invoke } from '@tauri-apps/api/core'
import type {
  SystemInfo,
  WindowInfo,
  DirectoryListing,
  FileInfo,
} from '../types/system'

// System Information
export const getSystemInfo = async (): Promise<SystemInfo> => {
  return await invoke('get_system_info')
}

export const getAppDataDir = async (): Promise<string> => {
  return await invoke('get_app_data_dir')
}

export const getAppLogDir = async (): Promise<string> => {
  return await invoke('get_app_log_dir')
}

// Notifications
export const sendNotification = async (
  title: string,
  body: string
): Promise<string> => {
  return await invoke('send_notification', { title, body })
}

// Window Management
export const getWindowInfo = async (): Promise<WindowInfo> => {
  return await invoke('get_window_info')
}

export const toggleWindowMaximize = async (): Promise<string> => {
  return await invoke('toggle_window_maximize')
}

export const minimizeWindow = async (): Promise<string> => {
  return await invoke('minimize_window')
}

export const centerWindow = async (): Promise<string> => {
  return await invoke('center_window')
}

export const setWindowTitle = async (title: string): Promise<string> => {
  return await invoke('set_window_title', { title })
}

export const createNewWindow = async (
  label: string,
  url: string
): Promise<string> => {
  return await invoke('create_new_window', { label, url })
}

// Command Execution
export const executeCommand = async (
  command: string,
  args: string[] = []
): Promise<string> => {
  return await invoke('execute_command', { command, args })
}

// File System Operations
export const readTextFile = async (path: string): Promise<string> => {
  return await invoke('read_text_file', { path })
}

export const writeTextFile = async (
  path: string,
  content: string
): Promise<string> => {
  return await invoke('write_text_file', { path, content })
}

export const appendTextFile = async (
  path: string,
  content: string
): Promise<string> => {
  return await invoke('append_text_file', { path, content })
}

export const deleteFile = async (path: string): Promise<string> => {
  return await invoke('delete_file', { path })
}

export const createDirectory = async (path: string): Promise<string> => {
  return await invoke('create_directory', { path })
}

export const listDirectory = async (
  path: string
): Promise<DirectoryListing> => {
  return await invoke('list_directory', { path })
}

export const fileExists = async (path: string): Promise<boolean> => {
  return await invoke('file_exists', { path })
}

export const getFileInfo = async (path: string): Promise<FileInfo> => {
  return await invoke('get_file_info', { path })
}

export const copyFile = async (
  source: string,
  destination: string
): Promise<string> => {
  return await invoke('copy_file', { source, destination })
}

export const moveFile = async (
  source: string,
  destination: string
): Promise<string> => {
  return await invoke('move_file', { source, destination })
}

// Utility functions for common operations
export const formatFileSize = (bytes: number): string => {
  const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB']
  if (bytes === 0) return '0 Bytes'
  const i = Math.floor(Math.log(bytes) / Math.log(1024))
  return Math.round((bytes / Math.pow(1024, i)) * 100) / 100 + ' ' + sizes[i]
}

export const getFileExtension = (filename: string): string => {
  return filename.slice(((filename.lastIndexOf('.') - 1) >>> 0) + 2)
}

export const getFileName = (path: string): string => {
  return path.split(/[\\/]/).pop() || ''
}

export const getParentDirectory = (path: string): string => {
  const parts = path.split(/[\\/]/)
  parts.pop()
  return parts.join('/')
}
