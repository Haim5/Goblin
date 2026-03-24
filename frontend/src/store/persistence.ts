import type { Network } from '../types/network'

const VERSION = '1.0'
const AUTOSAVE_KEY = 'gooph_autosave'

interface SaveFile {
  version: string
  created_at: string
  updated_at: string
  network: Network
}

export function serializeNetwork(network: Network): string {
  const file: SaveFile = {
    version: VERSION,
    created_at: new Date().toISOString(),
    updated_at: new Date().toISOString(),
    network,
  }
  return JSON.stringify(file, null, 2)
}

export function deserializeNetwork(json: string): Network | Error {
  try {
    const parsed = JSON.parse(json) as SaveFile
    if (!parsed.version) {
      return new Error('Invalid file format: missing version')
    }
    if (!parsed.network) {
      return new Error('Invalid file format: missing network')
    }
    return parsed.network
  } catch (e) {
    return new Error(`Failed to parse file: ${e instanceof Error ? e.message : String(e)}`)
  }
}

export function autosave(network: Network): void {
  try {
    localStorage.setItem(AUTOSAVE_KEY, serializeNetwork(network))
  } catch {
    // ignore storage errors
  }
}

export function loadAutosave(): Network | null {
  try {
    const json = localStorage.getItem(AUTOSAVE_KEY)
    if (!json) return null
    const result = deserializeNetwork(json)
    if (result instanceof Error) return null
    return result
  } catch {
    return null
  }
}

export function downloadFile(content: string, filename: string, mimeType: string): void {
  const blob = new Blob([content], { type: mimeType })
  const url = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  a.click()
  URL.revokeObjectURL(url)
}
