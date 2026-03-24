import { useRef, useEffect, useCallback } from 'react'
import { useNetworkStore } from '../store/networkStore'
import { serializeNetwork, deserializeNetwork, autosave, downloadFile } from '../store/persistence'

export function PersistenceBar() {
  const network = useNetworkStore((s) => s.network)
  const fileInputRef = useRef<HTMLInputElement>(null)

  useEffect(() => {
    const timer = setTimeout(() => autosave(network), 2000)
    return () => clearTimeout(timer)
  }, [network])

  const handleExportJson = useCallback(() => {
    downloadFile(serializeNetwork(network), 'network.json', 'application/json')
  }, [network])

  const handleExportSvg = useCallback(() => {
    const svg = document.querySelector('div[data-diagram] svg') as SVGElement | null
    if (!svg) {
      alert('Generate the schematic first.')
      return
    }
    const serializer = new XMLSerializer()
    const content = `<?xml version="1.0" encoding="utf-8"?>\n${serializer.serializeToString(svg)}`
    downloadFile(content, 'schematic.svg', 'image/svg+xml')
  }, [])

  const handleImport = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0]
    if (!file) return
    const reader = new FileReader()
    reader.onload = (ev) => {
      const json = ev.target?.result
      if (typeof json !== 'string') return
      const result = deserializeNetwork(json)
      if (result instanceof Error) {
        alert(`Import failed: ${result.message}`)
        return
      }
      useNetworkStore.setState({ network: result, past: [], future: [] })
    }
    reader.readAsText(file)
    e.target.value = ''
  }, [])

  return (
    <div style={{ display: 'flex', gap: 6, alignItems: 'center' }}>
      <button
        onClick={handleExportJson}
        style={{ padding: '4px 10px', fontSize: 12, border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', background: '#f0f0f0' }}
      >
        Export JSON
      </button>
      <button
        onClick={() => fileInputRef.current?.click()}
        style={{ padding: '4px 10px', fontSize: 12, border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', background: '#f0f0f0' }}
      >
        Import JSON
      </button>
      <button
        onClick={handleExportSvg}
        style={{ padding: '4px 10px', fontSize: 12, border: '1px solid #ccc', borderRadius: 3, cursor: 'pointer', background: '#f0f0f0' }}
      >
        Export SVG
      </button>
      <input ref={fileInputRef} type="file" accept=".json" style={{ display: 'none' }} onChange={handleImport} />
    </div>
  )
}
