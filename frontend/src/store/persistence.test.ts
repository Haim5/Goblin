import { describe, it, expect } from 'vitest'
import { serializeNetwork, deserializeNetwork } from './persistence'
import type { Network } from '../types/network'

const sampleNetwork: Network = {
  stations: {
    'abc-123': {
      id: 'abc-123',
      name: 'Central',
      geo: { lat: 51.5, lon: -0.1 },
      metadata: {},
    },
  },
  edges: {},
  lines: {},
}

describe('persistence', () => {
  it('round-trips network correctly', () => {
    const json = serializeNetwork(sampleNetwork)
    const result = deserializeNetwork(json)
    expect(result instanceof Error).toBe(false)
    if (result instanceof Error) return
    expect(result.stations['abc-123'].name).toBe('Central')
    expect(result.stations['abc-123'].geo.lat).toBe(51.5)
  })

  it('includes version in serialized output', () => {
    const json = serializeNetwork(sampleNetwork)
    const parsed = JSON.parse(json) as { version: string }
    expect(parsed.version).toBe('1.0')
  })

  it('returns Error for invalid JSON', () => {
    const result = deserializeNetwork('not-json')
    expect(result instanceof Error).toBe(true)
  })

  it('returns Error for JSON missing network field', () => {
    const result = deserializeNetwork(JSON.stringify({ version: '1.0' }))
    expect(result instanceof Error).toBe(true)
  })

  it('empty network round-trips correctly', () => {
    const empty: Network = { stations: {}, edges: {}, lines: {} }
    const json = serializeNetwork(empty)
    const result = deserializeNetwork(json)
    expect(result instanceof Error).toBe(false)
    if (result instanceof Error) return
    expect(Object.keys(result.stations).length).toBe(0)
  })
})
