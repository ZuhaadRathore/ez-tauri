// System and Tauri feature types

export interface SystemInfo {
  platform: string
  arch: string
  version: string
  hostname: string
}

export interface WindowInfo {
  label: string
  title: string
  isMaximized: boolean
  isMinimized: boolean
  isVisible: boolean
  isFocused: boolean
  position: [number, number]
  size: [number, number]
}

export interface FileInfo {
  name: string
  path: string
  size: number
  isDir: boolean
  isFile: boolean
  modified?: string
  created?: string
}

export interface DirectoryListing {
  path: string
  entries: FileInfo[]
}

export interface NotificationOptions {
  title: string
  body: string
  icon?: string
}

export interface CommandResult {
  success: boolean
  output?: string
  error?: string
}
