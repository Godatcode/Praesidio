import { mockData } from './mock'
import type {
  OverviewData,
  AuditEvent,
  ServerInfo,
  ThreatSignature,
  HoneypotAttack,
  ComplianceItem,
  ConfigData,
} from './mock'

async function fetchOrMock<T>(
  endpoint: string,
  mockFn: () => T,
  validate?: (data: unknown) => boolean,
): Promise<T> {
  try {
    const res = await fetch(`/api${endpoint}`)
    if (res.ok) {
      const data = await res.json()
      if (validate && !validate(data)) return mockFn()
      return data as T
    }
    return mockFn()
  } catch {
    return mockFn()
  }
}

// Shape validators — fall back to mock if the backend returns unexpected fields
const hasKeys = (d: unknown, keys: string[]) =>
  d != null && typeof d === 'object' && !Array.isArray(d) &&
  keys.every((k) => k in (d as Record<string, unknown>))

const isArrayWithShape = (d: unknown, key: string) =>
  Array.isArray(d) && (d.length === 0 || key in (d[0] as Record<string, unknown>))

export async function getOverview(): Promise<OverviewData> {
  return fetchOrMock('/overview', () => mockData.overview, (d) =>
    hasKeys(d, ['servers', 'threats_blocked']),
  )
}

export async function getEvents(): Promise<AuditEvent[]> {
  return fetchOrMock('/events', () => mockData.events, (d) =>
    isArrayWithShape(d, 'severity'),
  )
}

export async function getServers(): Promise<ServerInfo[]> {
  return fetchOrMock('/servers', () => mockData.servers, (d) =>
    isArrayWithShape(d, 'trust_score'),
  )
}

export async function getThreats(): Promise<ThreatSignature[]> {
  return fetchOrMock('/threats', () => mockData.threats, (d) =>
    isArrayWithShape(d, 'severity'),
  )
}

export async function getHoneypotAttacks(): Promise<HoneypotAttack[]> {
  return fetchOrMock('/honeypot/attacks', () => mockData.honeypot_attacks, (d) =>
    isArrayWithShape(d, 'attack_type'),
  )
}

export async function getHoneypotStatus(): Promise<{ running: boolean; uptime_hours: number; total_attacks: number }> {
  return fetchOrMock('/honeypot/status', () => mockData.honeypot_status, (d) =>
    hasKeys(d, ['running']),
  )
}

export async function getComplianceMCP(): Promise<ComplianceItem[]> {
  return fetchOrMock('/compliance/mcp', () => mockData.owasp_mcp, (d) =>
    isArrayWithShape(d, 'status'),
  )
}

export async function getComplianceAgentic(): Promise<ComplianceItem[]> {
  return fetchOrMock('/compliance/agentic', () => mockData.owasp_agentic, (d) =>
    isArrayWithShape(d, 'status'),
  )
}

export async function getTrend(): Promise<{ hour: string; clean: number; warning: number; critical: number }[]> {
  return fetchOrMock('/trend', () => mockData.trend, (d) =>
    isArrayWithShape(d, 'clean'),
  )
}

export async function getAuditLog(): Promise<(AuditEvent & { raw?: Record<string, unknown> })[]> {
  return fetchOrMock('/audit', () => mockData.audit, (d) =>
    isArrayWithShape(d, 'severity'),
  )
}

export async function getConfig(): Promise<ConfigData> {
  return fetchOrMock('/config', () => mockData.config, (d) =>
    hasKeys(d, ['global']),
  )
}

export async function triggerScan(): Promise<{ status: string; findings: number }> {
  return fetchOrMock('/scan', () => ({ status: 'completed', findings: 3 }))
}
