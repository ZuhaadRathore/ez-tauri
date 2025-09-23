/* eslint-env node */
import process from 'node:process'
import path from 'node:path'
import net from 'node:net'
import fs from 'node:fs'
import { fileURLToPath } from 'node:url'
import { spawn, type ChildProcess } from 'node:child_process'
import type { Options } from '@wdio/types'

const __dirname = path.dirname(fileURLToPath(import.meta.url))

const TAURI_DRIVER_PORT = Number(process.env.TAURI_DRIVER_PORT ?? 9516)
const EDGE_DRIVER_PORT = Number(process.env.EDGE_DRIVER_PORT ?? 9515)

let driverProcess: ChildProcess | undefined

function resolveTauriProductName() {
  if (process.env.TAURI_BINARY_NAME) return process.env.TAURI_BINARY_NAME
  try {
    const configPath = path.resolve(__dirname, 'src-tauri', 'tauri.conf.json')
    const raw = fs.readFileSync(configPath, 'utf-8')
    const parsed = JSON.parse(raw)
    if (
      typeof parsed?.productName === 'string' &&
      parsed.productName.trim().length > 0
    ) {
      return parsed.productName.trim()
    }
  } catch (error) {
    console.warn(
      '[wdio] unable to read tauri.conf.json to infer productName',
      error
    )
  }
  return 'tavuc-boilerplate'
}

function resolveTauriBinaryPath() {
  const explicit = process.env.TAURI_APP_PATH
  if (explicit) return explicit

  const profileDir =
    process.env.TAURI_TEST_PROFILE === 'release' ? 'release' : 'debug'
  const targetDir = path.resolve(__dirname, 'src-tauri', 'target', profileDir)
  const productName = resolveTauriProductName()

  const candidates: string[] = []
  if (process.platform === 'win32') {
    candidates.push(path.join(targetDir, `${productName}.exe`))
    candidates.push(path.join(targetDir, productName))
  } else if (process.platform === 'darwin') {
    candidates.push(
      path.join(
        targetDir,
        `${productName}.app`,
        'Contents',
        'MacOS',
        productName
      )
    )
    candidates.push(path.join(targetDir, productName))
  } else {
    candidates.push(path.join(targetDir, productName))
  }

  const found = candidates.find(candidate => fs.existsSync(candidate))
  return found ?? candidates[0]
}

function resolveTauriDriverBinary() {
  if (process.env.TAURI_DRIVER_BINARY) return process.env.TAURI_DRIVER_BINARY
  const isWin = process.platform === 'win32'
  const cargoHome =
    process.env.CARGO_HOME ||
    (isWin
      ? path.join(process.env.USERPROFILE ?? '', '.cargo')
      : path.join(process.env.HOME ?? '', '.cargo'))
  const candidate = isWin
    ? path.join(cargoHome, 'bin', 'tauri-driver.exe')
    : path.join(cargoHome, 'bin', 'tauri-driver')
  return candidate
}

function resolveBundledEdgeDriverBinary() {
  const executable =
    process.platform === 'win32' ? 'msedgedriver.exe' : 'msedgedriver'
  const candidate = path.resolve(
    __dirname,
    'tests',
    'desktop',
    'drivers',
    executable
  )
  return fs.existsSync(candidate) ? candidate : undefined
}

async function resolveEdgeDriverBinary() {
  if (process.env.EDGE_DRIVER_BINARY) return process.env.EDGE_DRIVER_BINARY
  const bundled = resolveBundledEdgeDriverBinary()
  if (bundled) return bundled
  try {
    const { download } = await import('edgedriver')
    return await download()
  } catch (error) {
    throw new Error(
      'Unable to resolve Microsoft Edge WebDriver binary. Provide EDGE_DRIVER_BINARY or place msedgedriver in tests/desktop/drivers. Original error: ' +
        (error instanceof Error ? error.message : String(error))
    )
  }
}

function waitForPort(port: number, timeout = 20000): Promise<void> {
  return new Promise((resolve, reject) => {
    const start = Date.now()
    const check = () => {
      const socket = net.createConnection({ port, host: '127.0.0.1' })
      socket.once('connect', () => {
        socket.destroy()
        resolve()
      })
      socket.once('error', () => {
        socket.destroy()
        if (Date.now() - start >= timeout) {
          reject(
            new Error(
              `tauri-driver port ${port} did not open within ${timeout}ms`
            )
          )
        } else {
          setTimeout(check, 250)
        }
      })
    }
    check()
  })
}

export const config: Options.Testrunner = {
  runner: 'local',
  specs: ['./tests/desktop/**/*.spec.ts'],
  maxInstances: 1,
  logLevel: process.env.WDIO_LOG_LEVEL ?? 'info',
  bail: 0,
  waitforTimeout: 10000,
  connectionRetryTimeout: 120000,
  connectionRetryCount: 1,
  framework: 'mocha',
  reporters: ['spec'],
  mochaOpts: {
    timeout: 60000,
  },
  autoCompileOpts: {
    tsNodeOpts: {
      transpileOnly: true,
      project: 'tsconfig.json',
    },
  },
  hostname: '127.0.0.1',
  port: TAURI_DRIVER_PORT,
  path: '/',
  capabilities: [
    {
      browserName: 'webview2',
      'tauri:options': {
        application: resolveTauriBinaryPath(),
        args: [],
      },
    },
  ],
  async onPrepare() {
    const edgeDriverPath = await resolveEdgeDriverBinary()
    const driverBinary = resolveTauriDriverBinary()

    driverProcess = spawn(
      driverBinary,
      [
        '--port',
        String(TAURI_DRIVER_PORT),
        '--native-port',
        String(EDGE_DRIVER_PORT),
        '--native-driver',
        edgeDriverPath,
      ],
      {
        stdio: 'inherit',
      }
    )

    driverProcess.on('error', error => {
      console.error('[tauri-driver] failed to start', error)
    })

    const shutdown = () => {
      if (driverProcess && !driverProcess.killed) {
        driverProcess.kill('SIGINT')
      }
    }

    process.on('exit', shutdown)
    process.on('SIGINT', shutdown)
    process.on('SIGTERM', shutdown)

    await waitForPort(TAURI_DRIVER_PORT)
  },
  async onComplete() {
    if (driverProcess && !driverProcess.killed) {
      driverProcess.kill('SIGINT')
    }
  },
}
