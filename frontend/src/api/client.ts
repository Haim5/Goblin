import type { LayoutRequest, LayoutResult, Network, ValidationResponse } from '../types/network'

export async function postLayout(req: LayoutRequest): Promise<LayoutResult> {
  const res = await fetch('/api/layout', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(req),
  })
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: 'Unknown error' }))
    throw new Error((body as { error: string }).error ?? 'Layout request failed')
  }
  return res.json() as Promise<LayoutResult>
}

export async function postValidate(network: Network): Promise<ValidationResponse> {
  const res = await fetch('/api/validate', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ network }),
  })
  if (!res.ok) {
    const body = await res.json().catch(() => ({ error: 'Unknown error' }))
    throw new Error((body as { error: string }).error ?? 'Validation request failed')
  }
  return res.json() as Promise<ValidationResponse>
}
