#!/usr/bin/env node
import { appendFileSync, readFileSync } from "node:fs"
import { join } from "node:path"

const workspaceRoot = process.cwd()

const packageJsonPath = join(workspaceRoot, "package.json")
const cargoTomlPath = join(workspaceRoot, "src-tauri", "Cargo.toml")

const packageJson = JSON.parse(readFileSync(packageJsonPath, "utf8"))
const packageVersion = packageJson.version
if (!packageVersion) {
  throw new Error("Unable to resolve version field from package.json")
}

const cargoToml = readFileSync(cargoTomlPath, "utf8")
const cargoMatch = cargoToml.match(/^version\s*=\s*"([^"]+)"/m)
if (!cargoMatch) {
  throw new Error("Unable to resolve version field from src-tauri/Cargo.toml")
}

const cargoVersion = cargoMatch[1]
if (cargoVersion !== packageVersion) {
  throw new Error(
    `Version mismatch between package.json (${packageVersion}) and Cargo.toml (${cargoVersion})`
  )
}

const refName = process.env.GITHUB_REF_NAME ?? ""
if (refName) {
  const normalizedTag = refName.replace(/^v/, "")
  if (normalizedTag !== packageVersion) {
    throw new Error(
      `Git tag (${refName}) does not match project version (${packageVersion})`
    )
  }
}

const outputPath = process.env.GITHUB_OUTPUT
if (outputPath) {
  appendFileSync(outputPath, `version=${packageVersion}\n`)
}

console.log(`Release version verified: v${packageVersion}`)