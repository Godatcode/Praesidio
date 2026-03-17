import { useState } from 'react'
import { useApi } from '../hooks/useApi'
import { getThreats } from '../api/client'
import { ThreatCard } from '../components/ThreatCard'
import { useReveal } from '../hooks/useReveal'
import { Search } from 'lucide-react'
import type { Severity } from '../api/mock'

const filters: { label: string; value: Severity | 'all' }[] = [
  { label: 'All', value: 'all' },
  { label: 'Critical', value: 'critical' },
  { label: 'High', value: 'high' },
  { label: 'Medium', value: 'medium' },
  { label: 'Warning', value: 'warning' },
  { label: 'Low', value: 'low' },
]

export function Threats() {
  const { data: threats } = useApi(getThreats)
  const [search, setSearch] = useState('')
  const [severity, setSeverity] = useState<Severity | 'all'>('all')
  const revealHeader = useReveal()
  const revealFilters = useReveal()
  const revealList = useReveal()

  const filtered = (threats ?? []).filter((t) => {
    if (severity !== 'all' && t.severity !== severity) return false
    if (search) {
      const q = search.toLowerCase()
      return (
        t.title.toLowerCase().includes(q) ||
        t.id.toLowerCase().includes(q) ||
        t.description.toLowerCase().includes(q) ||
        t.category.toLowerCase().includes(q)
      )
    }
    return true
  })

  return (
    <div>
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <h1 className="page-title">Threat Signatures</h1>
        <p className="page-subtitle">{threats?.length ?? 0} detection rules active</p>
      </div>

      <div ref={revealFilters} className="reveal" style={{ marginBottom: 24, display: 'flex', flexDirection: 'column', gap: 12 }}>
        <div style={{ position: 'relative' }}>
          <Search size={14} style={{ position: 'absolute', left: 12, top: '50%', transform: 'translateY(-50%)', color: 'rgba(255,255,255,0.2)' }} />
          <input
            type="text"
            placeholder="Search threats..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            style={{ width: '100%', paddingLeft: 36, paddingRight: 16, paddingTop: 10, paddingBottom: 10 }}
          />
        </div>

        <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
          {filters.map((f) => (
            <button
              key={f.value}
              onClick={() => setSeverity(f.value)}
              style={{
                padding: '6px 12px',
                borderRadius: 8,
                fontSize: 12,
                fontWeight: 450,
                border: severity === f.value ? '1px solid rgba(255,255,255,0.08)' : '1px solid transparent',
                background: severity === f.value ? 'rgba(255,255,255,0.06)' : 'transparent',
                color: severity === f.value ? 'rgba(255,255,255,0.9)' : 'rgba(255,255,255,0.4)',
                cursor: 'pointer',
                transition: 'all 0.15s ease',
              }}
            >
              {f.label}
            </button>
          ))}
          {filtered.length !== (threats?.length ?? 0) && (
            <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.2)', marginLeft: 8, fontFamily: 'var(--font-mono)', fontVariantNumeric: 'tabular-nums' }}>
              {filtered.length} results
            </span>
          )}
        </div>
      </div>

      <div ref={revealList} className="reveal" style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
        {filtered.map((threat) => (
          <ThreatCard key={threat.id} threat={threat} />
        ))}
        {filtered.length === 0 && threats && (
          <div style={{ textAlign: 'center', padding: '48px 0', color: 'rgba(255,255,255,0.2)', fontSize: 13 }}>
            No threats match your filters
          </div>
        )}
      </div>
    </div>
  )
}
