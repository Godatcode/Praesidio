import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { cn, severityColor } from '../lib/utils'
import { SeverityBadge } from './SeverityBadge'
import { ChevronDown } from 'lucide-react'
import type { ThreatSignature } from '../api/mock'

interface ThreatCardProps {
  threat: ThreatSignature
}

export function ThreatCard({ threat }: ThreatCardProps) {
  const [expanded, setExpanded] = useState(false)
  const colors = severityColor(threat.severity)

  return (
    <div className="card" style={{ overflow: 'hidden' }}>
      <button
        onClick={() => setExpanded(!expanded)}
        style={{ width: '100%', textAlign: 'left', padding: 20, background: 'none', border: 'none', cursor: 'pointer', color: 'inherit' }}
      >
        <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between', gap: 12 }}>
          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 6 }}>
              <span style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontVariantNumeric: 'tabular-nums' }}>{threat.id}</span>
              <SeverityBadge severity={threat.severity} />
              <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', padding: '2px 8px', background: 'rgba(255,255,255,0.06)', borderRadius: 4 }}>
                {threat.category}
              </span>
            </div>
            <h3 style={{ fontSize: 13, fontWeight: 450, color: 'rgba(255,255,255,0.9)' }}>{threat.title}</h3>
            <p style={{ fontSize: 12, color: 'rgba(255,255,255,0.3)', marginTop: 4, overflow: 'hidden', display: '-webkit-box', WebkitLineClamp: 2, WebkitBoxOrient: 'vertical' as const }}>{threat.description}</p>
          </div>

          <motion.div
            animate={{ rotate: expanded ? 180 : 0 }}
            transition={{ duration: 0.2 }}
            style={{ flexShrink: 0, marginTop: 4 }}
          >
            <ChevronDown size={14} style={{ color: 'rgba(255,255,255,0.2)' }} />
          </motion.div>
        </div>

        {threat.owasp_ids.length > 0 && (
          <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginTop: 8 }}>
            {threat.owasp_ids.map((id) => (
              <span
                key={id}
                className={cn('font-mono', colors.text, colors.bg)}
                style={{ fontSize: 10, padding: '2px 6px', borderRadius: 4 }}
              >
                {id}
              </span>
            ))}
          </div>
        )}
      </button>

      <AnimatePresence>
        {expanded && (
          <motion.div
            initial={{ height: 0, opacity: 0 }}
            animate={{ height: 'auto', opacity: 1 }}
            exit={{ height: 0, opacity: 0 }}
            transition={{ duration: 0.3, ease: [0.4, 0, 0.2, 1] }}
            style={{ overflow: 'hidden' }}
          >
            <div style={{ padding: '12px 20px 20px', borderTop: '1px solid rgba(255,255,255,0.08)', display: 'flex', flexDirection: 'column', gap: 16 }}>
              {threat.pattern && (
                <div>
                  <span className="section-label">Detection Pattern</span>
                  <code style={{
                    display: 'block',
                    marginTop: 6,
                    fontFamily: 'var(--font-mono)',
                    fontSize: 11,
                    color: 'var(--purple)',
                    background: 'rgba(139,92,246,0.06)',
                    padding: '8px 12px',
                    borderRadius: 8,
                  }}>
                    {threat.pattern}
                  </code>
                </div>
              )}

              {threat.mitigations.length > 0 && (
                <div>
                  <span className="section-label">Mitigations</span>
                  <ul style={{ marginTop: 6, display: 'flex', flexDirection: 'column', gap: 4 }}>
                    {threat.mitigations.map((m, i) => (
                      <li key={i} style={{ fontSize: 12, color: 'rgba(255,255,255,0.5)', display: 'flex', alignItems: 'center', gap: 8 }}>
                        <span style={{ width: 4, height: 4, borderRadius: '50%', background: '#22c55e', flexShrink: 0 }} />
                        {m}
                      </li>
                    ))}
                  </ul>
                </div>
              )}

              {threat.references.length > 0 && (
                <div>
                  <span className="section-label">References</span>
                  <div style={{ marginTop: 6, display: 'flex', flexWrap: 'wrap', gap: 6 }}>
                    {threat.references.map((ref, i) => (
                      <span key={i} style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.3)', background: 'rgba(255,255,255,0.06)', padding: '2px 8px', borderRadius: 4 }}>
                        {ref}
                      </span>
                    ))}
                  </div>
                </div>
              )}
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  )
}
