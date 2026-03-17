import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { useApi } from '../hooks/useApi'
import { getAuditLog } from '../api/client'
import { SeverityBadge } from '../components/SeverityBadge'
import { CodeBlock } from '../components/CodeBlock'
import { useRelativeTime } from '../hooks/useRelativeTime'
import { useReveal } from '../hooks/useReveal'
import { ChevronDown } from 'lucide-react'
import type { Severity } from '../api/mock'

const severityOptions: (Severity | 'all')[] = ['all', 'critical', 'high', 'warning', 'medium', 'low', 'info', 'clean']

export function AuditLog() {
  const { data: logs } = useApi(getAuditLog)
  const [severityFilter, setSeverityFilter] = useState<Severity | 'all'>('all')
  const [categoryFilter, setCategoryFilter] = useState<string>('all')
  const [serverFilter, setServerFilter] = useState<string>('all')
  const [expandedId, setExpandedId] = useState<string | null>(null)
  const revealHeader = useReveal()
  const revealFilters = useReveal()
  const revealTable = useReveal()

  const categories = Array.from(new Set(logs?.map((l) => l.category) ?? []))
  const servers = Array.from(new Set(logs?.map((l) => l.server) ?? []))

  const filtered = (logs ?? []).filter((l) => {
    if (severityFilter !== 'all' && l.severity !== severityFilter) return false
    if (categoryFilter !== 'all' && l.category !== categoryFilter) return false
    if (serverFilter !== 'all' && l.server !== serverFilter) return false
    return true
  })

  return (
    <div>
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between' }}>
          <div>
            <h1 className="page-title">Audit Log</h1>
            <p className="page-subtitle">{logs?.length ?? 0} events recorded</p>
          </div>
          <button className="btn" style={{ marginTop: 12 }}>Export JSONL</button>
        </div>
      </div>

      <div ref={revealFilters} className="reveal" style={{ marginBottom: 24 }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
          <Select
            label="Severity"
            value={severityFilter}
            onChange={setSeverityFilter}
            options={severityOptions.map((s) => ({ label: s === 'all' ? 'All Severities' : s, value: s }))}
          />
          <Select
            label="Category"
            value={categoryFilter}
            onChange={setCategoryFilter}
            options={[{ label: 'All Categories', value: 'all' }, ...categories.map((c) => ({ label: c, value: c }))]}
          />
          <Select
            label="Server"
            value={serverFilter}
            onChange={setServerFilter}
            options={[{ label: 'All Servers', value: 'all' }, ...servers.map((s) => ({ label: s, value: s }))]}
          />

          {(severityFilter !== 'all' || categoryFilter !== 'all' || serverFilter !== 'all') && (
            <button
              onClick={() => { setSeverityFilter('all'); setCategoryFilter('all'); setServerFilter('all') }}
              style={{ fontSize: 11, color: 'rgba(255,255,255,0.2)', background: 'none', border: 'none', cursor: 'pointer' }}
            >
              Clear filters
            </button>
          )}

          <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.2)', marginLeft: 'auto', fontFamily: 'var(--font-mono)', fontVariantNumeric: 'tabular-nums' }}>
            {filtered.length} entries
          </span>
        </div>
      </div>

      <div ref={revealTable} className="reveal">
        <div className="card" style={{ overflow: 'hidden' }}>
          <table style={{ width: '100%', borderCollapse: 'collapse' }}>
            <thead>
              <tr>
                {['Time', 'Severity', 'Server', 'Tool', 'Category', 'Description', ''].map((h) => (
                  <th key={h} style={{ textAlign: 'left', padding: '10px 16px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)', background: 'rgba(255,255,255,0.015)', ...(h === '' ? { width: 32 } : {}) }}>{h}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {filtered.map((log) => (
                <AuditRow
                  key={log.id}
                  log={log}
                  expanded={expandedId === log.id}
                  onToggle={() => setExpandedId(expandedId === log.id ? null : log.id)}
                />
              ))}
            </tbody>
          </table>

          {filtered.length === 0 && (
            <div style={{ textAlign: 'center', padding: '48px 0', color: 'rgba(255,255,255,0.2)', fontSize: 13 }}>
              No audit events match your filters
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

function AuditRow({
  log,
  expanded,
  onToggle,
}: {
  log: { id: string; timestamp: string; severity: string; server: string; tool?: string; category: string; description: string; raw?: Record<string, unknown> }
  expanded: boolean
  onToggle: () => void
}) {
  const timeAgo = useRelativeTime(log.timestamp)
  const tdStyle: React.CSSProperties = { padding: '10px 16px', borderBottom: '1px solid rgba(255,255,255,0.02)' }

  return (
    <>
      <tr
        onClick={onToggle}
        style={{ transition: 'background 0.1s ease', cursor: 'pointer' }}
        onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.02)'}
        onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
      >
        <td style={tdStyle}><span style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontVariantNumeric: 'tabular-nums' }}>{timeAgo}</span></td>
        <td style={tdStyle}><SeverityBadge severity={log.severity} /></td>
        <td style={tdStyle}><span style={{ fontSize: 12, color: 'rgba(255,255,255,0.5)' }}>{log.server}</span></td>
        <td style={tdStyle}><span style={{ fontFamily: 'var(--font-mono)', fontSize: 12, color: 'rgba(255,255,255,0.9)' }}>{log.tool ?? '\u2014'}</span></td>
        <td style={tdStyle}><span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>{log.category}</span></td>
        <td style={tdStyle}><span style={{ fontSize: 12, color: 'rgba(255,255,255,0.3)', overflow: 'hidden', display: '-webkit-box', WebkitLineClamp: 1, WebkitBoxOrient: 'vertical' as const }}>{log.description}</span></td>
        <td style={tdStyle}>
          <motion.div animate={{ rotate: expanded ? 180 : 0 }} transition={{ duration: 0.2 }}>
            <ChevronDown size={14} style={{ color: 'rgba(255,255,255,0.2)' }} />
          </motion.div>
        </td>
      </tr>
      <AnimatePresence>
        {expanded && log.raw && (
          <motion.tr initial={{ opacity: 0 }} animate={{ opacity: 1 }} exit={{ opacity: 0 }}>
            <td colSpan={7} style={{ padding: '12px 16px', background: 'rgba(0,0,0,0.2)' }}>
              <CodeBlock data={log.raw} />
            </td>
          </motion.tr>
        )}
      </AnimatePresence>
    </>
  )
}

function Select<T extends string>({
  label,
  value,
  onChange,
  options,
}: {
  label: string
  value: T
  onChange: (v: T) => void
  options: { label: string; value: T }[]
}) {
  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value as T)}
      aria-label={label}
      style={{ fontFamily: 'var(--font-mono)', fontSize: 11 }}
    >
      {options.map((o) => (
        <option key={o.value} value={o.value}>
          {o.label}
        </option>
      ))}
    </select>
  )
}
