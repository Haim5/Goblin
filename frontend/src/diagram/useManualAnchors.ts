import { useState, useCallback } from 'react'
import type { StationId, Point2D } from '../types/network'

export function useManualAnchors() {
  const [anchors, setAnchors] = useState<Record<StationId, Point2D>>({})

  const setAnchor = useCallback((id: StationId, position: Point2D) => {
    setAnchors((prev) => ({ ...prev, [id]: position }))
  }, [])

  const clearAnchor = useCallback((id: StationId) => {
    setAnchors((prev) => {
      const next = { ...prev }
      delete next[id]
      return next
    })
  }, [])

  const clearAll = useCallback(() => setAnchors({}), [])

  const toArray = useCallback(() =>
    Object.entries(anchors).map(([station_id, position]) => ({ station_id, position })),
    [anchors]
  )

  return { anchors, setAnchor, clearAnchor, clearAll, toArray }
}
