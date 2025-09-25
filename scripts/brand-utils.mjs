#!/usr/bin/env node

import { execSync } from 'node:child_process'

// Colors and styling
export const colors = {
  reset: '\x1b[0m',
  bright: '\x1b[1m',
  dim: '\x1b[2m',
  underscore: '\x1b[4m',
  blink: '\x1b[5m',
  reverse: '\x1b[7m',
  hidden: '\x1b[8m',

  // Foreground colors
  black: '\x1b[30m',
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  magenta: '\x1b[35m',
  cyan: '\x1b[36m',
  white: '\x1b[37m',

  // Bright foreground colors
  brightBlack: '\x1b[90m',
  brightRed: '\x1b[91m',
  brightGreen: '\x1b[92m',
  brightYellow: '\x1b[93m',
  brightBlue: '\x1b[94m',
  brightMagenta: '\x1b[95m',
  brightCyan: '\x1b[96m',
  brightWhite: '\x1b[97m',

  // Background colors
  bgBlack: '\x1b[40m',
  bgRed: '\x1b[41m',
  bgGreen: '\x1b[42m',
  bgYellow: '\x1b[43m',
  bgBlue: '\x1b[44m',
  bgMagenta: '\x1b[45m',
  bgCyan: '\x1b[46m',
  bgWhite: '\x1b[47m'
}

// Simple 3D EZ-Tauri logo
export const ezTauriLogo = `
${colors.brightCyan}███████╗███████╗    ████████╗ █████╗ ██╗   ██╗██████╗ ██╗
${colors.cyan}██╔════╝╚══███╔╝    ╚══██╔══╝██╔══██╗██║   ██║██╔══██╗██║
${colors.brightBlue}█████╗    ███╔╝ █████╗ ██║   ███████║██║   ██║██████╔╝██║
${colors.blue}██╔══╝   ███╔╝  ╚════╝ ██║   ██╔══██║██║   ██║██╔══██╗██║
${colors.brightMagenta}███████╗███████╗       ██║   ██║  ██║╚██████╔╝██║  ██║██║
${colors.magenta}╚══════╝╚══════╝       ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝╚═╝${colors.reset}
`

// Simple logo display function
export function showLogo() {
  console.log(ezTauriLogo)
}

export default {
  colors,
  ezTauriLogo,
  showLogo
}