import { useCallback, useState, useEffect } from 'react'
import { MapView } from './map/MapView'
import { Toolbar } from './editor/Toolbar'
import { LinePanel } from './editor/LinePanel'
import { PropertiesPanel } from './editor/PropertiesPanel'
import { DiagramView } from './diagram/DiagramView'
import { useManualAnchors } from './diagram/useManualAnchors'
import { useNetworkStore } from './store/networkStore'
import { useLayoutStore } from './store/layoutStore'
import { postLayout, postValidate } from './api/client'
import { loadAutosave } from './store/persistence'
import type { LayoutOptions, ValidationError } from './types/network'

const DEFAULT_OPTIONS: LayoutOptions = {
  time_budget_ms: 5000,
  min_station_spacing: 60,
  canvas_size: 1000,
  weights: {
    octilinear: 3.0,
    length: 1.0,
    bends: 2.0,
    separation: 4.0,
    crossings: 2.0,
    topology: 1.5,
    balance: 0.5,
  },
}

export function App() {
  const network = useNetworkStore((s) => s.network)
  const { setResult, setLoading, setError, loading } = useLayoutStore()
  const [validationErrors, setValidationErrors] = useState<ValidationError[]>([])
  const [validationPending, setValidationPending] = useState(false)
  const { anchors, setAnchor, clearAnchor, clearAll, toArray } = useManualAnchors()

  useEffect(() => {
    const saved = loadAutosave()
    if (saved && Object.keys(saved.stations).length > 0) {
      const confirmed = window.confirm('Restore previous session?')
      if (confirmed) {
        useNetworkStore.setState({ network: saved, past: [], future: [] })
      }
    }
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [])

  useEffect(() => {
    let timer: ReturnType<typeof setTimeout>
    setValidationPending(true)
    timer = setTimeout(async () => {
      try {
        const res = await postValidate(network)
        setValidationErrors(res.errors)
      } catch {
        // ignore validation errors on network failure
      } finally {
        setValidationPending(false)
      }
    }, 300)
    return () => clearTimeout(timer)
  }, [network])

  const handleGenerate = useCallback(async () => {
    setLoading(true)
    setError(null)
    try {
      const result = await postLayout({
        network,
        manual_anchors: toArray(),
        options: DEFAULT_OPTIONS,
      })
      setResult(result)
    } catch (e) {
      setError(e instanceof Error ? e.message : 'Unknown error')
    } finally {
      setLoading(false)
    }
  }, [network, toArray, setLoading, setError, setResult])

  const isValid = validationErrors.length === 0 && !validationPending

  return (
    <div style={{ display: 'flex', flexDirection: 'column', height: '100%', width: '100%' }}>
      <Toolbar onGenerateSchematic={handleGenerate} generating={loading} validationValid={isValid} />
      {validationErrors.length > 0 && (
        <div style={{ padding: '4px 12px', background: '#ffebee', borderBottom: '1px solid #ef9a9a', fontSize: 12, color: '#c62828' }}>
          {validationErrors.map((e, i) => <span key={i}>{e.message}{i < validationErrors.length - 1 ? ' · ' : ''}</span>)}
        </div>
      )}
      <div style={{ flex: 1, display: 'flex', overflow: 'hidden' }}>
        <div style={{ flex: 1, position: 'relative' }}>
          <MapView />
        </div>
        <div style={{ width: 240, display: 'flex', flexDirection: 'column', borderLeft: '1px solid #ddd', background: '#fff' }}>
          <div style={{ flex: 1, overflowY: 'auto', borderBottom: '1px solid #ddd' }}>
            <LinePanel />
          </div>
          <div style={{ height: 200, overflowY: 'auto' }}>
            <PropertiesPanel />
          </div>
        </div>
        <div style={{ width: 600, borderLeft: '1px solid #ddd' }}>
          <DiagramView
            anchors={anchors}
            onAnchorChange={setAnchor}
            onAnchorClear={clearAnchor}
            onAnchorClearAll={clearAll}
          />
        </div>
      </div>
    </div>
  )
}
