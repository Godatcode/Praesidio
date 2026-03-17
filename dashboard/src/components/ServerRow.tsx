import { useState } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import { cn, severityColor, trustScoreColor } from '../lib/utils'
import { useRelativeTime } from '../hooks/useRelativeTime'
import { SeverityBadge } from './SeverityBadge'
import { ChevronDown } from 'lucide-react'
import type { ServerInfo } from '../api/mock'

interface ServerRowProps {
  server: ServerInfo
  onScan?: (name: string) => void
}

export function ServerRow({ server, onScan }: ServerRowProps) {
  const [expanded, setExpanded] = useState(false)
  const lastScan = useRelativeTime(server.last_scan)

  const statusColor =
    server.status === 'active' ? '#22c55e' :
    server.status === 'blocked' ? '#ef4444' :
    server.status === 'scanning' ? '#f59e0b' :
    'rgba(255,255,255,0.15)'

  return (
    <div className="card" style={{ overflow: 'hidden' }}>
      <button
        onClick={() => setExpanded(!expanded)}
        style={{ width: '100%', textAlign: 'left', padding: 20, background: 'none', border: 'none', cursor: 'pointer', color: 'inherit' }}
      >
        <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
          <span style={{ width: 6, height: 6, borderRadius: '50%', background: statusColor, flexShrink: 0 }} />

          <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
              <span style={{ fontSize: 13, fontWeight: 450, color: 'rgba(255,255,255,0.9)' }}>
                {server.display_name}
              </span>
              {server.status === 'blocked' && <SeverityBadge severity="critical" />}
            </div>
            <span style={{ fontSize: 11, fontFamily: 'var(--font-mono)', color: 'rgba(255,255,255,0.2)' }}>{server.config_source}</span>
          </div>

          <div style={{ display: 'flex', alignItems: 'center', gap: 24, flexShrink: 0 }}>
            <div style={{ textAlign: 'right' }}>
              <div style={{ fontFamily: 'var(--font-mono)', fontSize: 13, fontVariantNumeric: 'tabular-nums', color: 'rgba(255,255,255,0.9)' }}>
                {server.tools.length}
              </div>
              <div style={{ fontSize: 10, color: 'rgba(255,255,255,0.2)' }}>tools</div>
            </div>

            <div style={{ width: 96 }}>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 4 }}>
                <span style={{ fontSize: 10, color: 'rgba(255,255,255,0.2)' }}>Trust</span>
                <span className={trustScoreColor(server.trust_score)} style={{ fontFamily: 'var(--font-mono)', fontSize: 11, fontVariantNumeric: 'tabular-nums' }}>
                  {server.trust_score}
                </span>
              </div>
              <div className="trust-bar" style={{ width: '100%' }}>
                <div
                  className={`trust-bar-fill ${server.trust_score >= 80 ? 'high' : server.trust_score >= 50 ? 'medium' : 'low'}`}
                  style={{ width: `${server.trust_score}%` }}
                />
              </div>
            </div>

            <span style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontVariantNumeric: 'tabular-nums', width: 64, textAlign: 'right' }}>
              {lastScan}
            </span>
          </div>

          <motion.div
            animate={{ rotate: expanded ? 180 : 0 }}
            transition={{ duration: 0.2 }}
          >
            <ChevronDown size={14} style={{ color: 'rgba(255,255,255,0.2)' }} />
          </motion.div>
        </div>
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
            <div style={{ borderTop: '1px solid rgba(255,255,255,0.08)', padding: 20 }}>
              <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
                <span className="section-label">Tools ({server.tools.length})</span>
                {onScan && (
                  <button
                    onClick={(e) => { e.stopPropagation(); onScan(server.name) }}
                    style={{ fontSize: 12, fontWeight: 450, color: 'var(--purple)', background: 'none', border: 'none', cursor: 'pointer' }}
                  >
                    Scan Now
                  </button>
                )}
              </div>

              <div style={{ display: 'flex', flexDirection: 'column', gap: 4 }}>
                {server.tools.map((tool) => {
                  const analysisColors = severityColor(
                    tool.llm_analysis === 'malicious' ? 'critical' :
                    tool.llm_analysis === 'suspicious' ? 'warning' :
                    tool.llm_analysis === 'pending' ? 'info' : 'clean'
                  )

                  return (
                    <div
                      key={tool.name}
                      style={{
                        display: 'flex',
                        alignItems: 'center',
                        gap: 12,
                        padding: '8px 12px',
                        borderRadius: 8,
                        transition: 'background 0.1s ease',
                      }}
                      onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.02)'}
                      onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
                    >
                      <span style={{ fontFamily: 'var(--font-mono)', fontSize: 12, color: 'rgba(255,255,255,0.9)', flex: 1, minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                        {tool.name}
                      </span>

                      <span className={cn(
                        'text-[10px] font-mono px-1.5 py-0.5 rounded',
                        tool.pinned ? 'text-emerald-400 bg-emerald-400/10' : 'text-red-400 bg-red-400/10',
                      )}>
                        {tool.pinned ? 'PINNED' : 'UNPINNED'}
                      </span>

                      <span className={cn(
                        'text-[10px] font-mono px-1.5 py-0.5 rounded',
                        tool.behavior_baseline ? 'text-blue-400 bg-blue-400/10' : 'text-zinc-500 bg-white/[0.06]',
                      )}>
                        {tool.behavior_baseline ? 'BASELINED' : 'NO BASELINE'}
                      </span>

                      <span className={cn(
                        'text-[10px] font-mono px-1.5 py-0.5 rounded uppercase',
                        analysisColors.text,
                        analysisColors.bg,
                      )}>
                        {tool.llm_analysis}
                      </span>

                      <span style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontVariantNumeric: 'tabular-nums', width: 32, textAlign: 'right' }}>
                        {tool.calls_last_24h}
                      </span>
                    </div>
                  )
                })}
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  )
}
