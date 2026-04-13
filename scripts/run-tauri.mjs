import { existsSync, readdirSync, readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { spawnSync } from 'node:child_process'

const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)
const projectRoot = path.resolve(__dirname, '..')
const srcTauriDir = path.join(projectRoot, 'src-tauri')
const targetBuildDir = path.join(srcTauriDir, 'target', 'debug', 'build')

function findStaleTargetPath() {
  if (!existsSync(targetBuildDir)) {
    return null
  }

  for (const entry of readdirSync(targetBuildDir, { withFileTypes: true })) {
    if (!entry.isDirectory()) {
      continue
    }

    const outputFile = path.join(targetBuildDir, entry.name, 'output')
    if (!existsSync(outputFile)) {
      continue
    }

    const content = readFileSync(outputFile, 'utf8')
    for (const line of content.split('\n')) {
      if (!line.startsWith('cargo:rerun-if-changed=')) {
        continue
      }

      if (!line.includes('/src-tauri/tauri.conf.json')) {
        continue
      }

      const configuredPath = line.slice('cargo:rerun-if-changed='.length).trim()
      if (!configuredPath.startsWith(srcTauriDir)) {
        return configuredPath
      }
    }
  }

  return null
}

const stalePath = findStaleTargetPath()
if (stalePath) {
  console.log(`Detected stale Tauri build cache from: ${stalePath}`)
  console.log('Running cargo clean to regenerate target files for the current project path...')

  const cleanResult = spawnSync('cargo', ['clean', '--manifest-path', path.join(srcTauriDir, 'Cargo.toml')], {
    cwd: projectRoot,
    stdio: 'inherit',
  })

  if (cleanResult.status !== 0) {
    process.exit(cleanResult.status ?? 1)
  }
}

const tauriArgs = process.argv.slice(2)
const tauriBin = path.join(projectRoot, 'node_modules', '.bin', 'tauri')
const result = spawnSync(tauriBin, tauriArgs, {
  cwd: projectRoot,
  stdio: 'inherit',
})

process.exit(result.status ?? 1)
