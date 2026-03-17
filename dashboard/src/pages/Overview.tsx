import { useState, useCallback } from 'react'
import { useApi } from '../hooks/useApi'
import { getOverview, getTrend, triggerScan } from '../api/client'
import { StatCard } from '../components/StatCard'
import { LiveFeed } from '../components/LiveFeed'
import { TrendChart } from '../components/TrendChart'
import { ProgressRing } from '../components/ProgressRing'
import { SeverityBadge } from '../components/SeverityBadge'
import { trustScoreColor } from '../lib/utils'
import { useReveal } from '../hooks/useReveal'
import { Scan } from 'lucide-react'

export function Overview() {
  const { data: overview } = useApi(getOverview)
  const { data: trend } = useApi(getTrend)
  const [scanning, setScanning] = useState(false)

  const revealHeader = useReveal()
  const revealMetrics = useReveal()
  const revealFeed = useReveal()
  const revealServers = useReveal()
  const revealConfigs = useReveal()

  const handleScan = useCallback(async () => {
    setScanning(true)
    try {
      await triggerScan()
    } finally {
      setTimeout(() => setScanning(false), 1500)
    }
  }, [])

  if (!overview) {
    return (
      <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: 256 }}>
        <div style={{
          width: 20, height: 20,
          border: '2px solid rgba(139,92,246,0.4)',
          borderTopColor: 'transparent',
          borderRadius: '50%',
          animation: 'spin 1s linear infinite',
        }} />
        <style>{`@keyframes spin { to { transform: rotate(360deg); } }`}</style>
      </div>
    )
  }

  const recentServers = [
    { name: 'GitHub MCP', tools: 5, status: 'active' as const, trust: 96, lastScan: '15m ago' },
    { name: 'Filesystem', tools: 4, status: 'active' as const, trust: 94, lastScan: '15m ago' },
    { name: 'PostgreSQL', tools: 3, status: 'active' as const, trust: 91, lastScan: '30m ago' },
    { name: 'Slack', tools: 4, status: 'active' as const, trust: 92, lastScan: '30m ago' },
    { name: 'Custom API Gateway', tools: 4, status: 'active' as const, trust: 73, lastScan: '18m ago' },
    { name: 'Sketchy Math Service', tools: 2, status: 'blocked' as const, trust: 4, lastScan: '48m ago' },
    { name: 'Internal DevTools', tools: 5, status: 'active' as const, trust: 88, lastScan: '2h ago' },
  ]

  return (
    <div>
      {/* Header — big type */}
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between' }}>
          <div>
            <h1 className="page-title">Overview</h1>
            <p className="page-subtitle">Real-time security monitoring for MCP servers</p>
          </div>
          <button
            onClick={handleScan}
            disabled={scanning}
            className="btn btn-primary"
            style={{ marginTop: 12, opacity: scanning ? 0.5 : 1 }}
          >
            {scanning ? (
              <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={{
                  width: 14, height: 14,
                  border: '2px solid rgba(139,92,246,0.4)',
                  borderTopColor: 'transparent',
                  borderRadius: '50%',
                  animation: 'spin 1s linear infinite',
                }} />
                Scanning...
              </span>
            ) : (
              <span style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <Scan size={14} />
                Scan Now
              </span>
            )}
          </button>
        </div>
      </div>

      {/* Metric cards */}
      <div ref={revealMetrics} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: 20 }}>
          <StatCard
            label="Servers"
            value={overview.servers ?? 0}
            trend={{ value: 16, positive: true }}
            sparklineData={overview.sparklines?.servers}
            sparklineColor="#8b5cf6"
          />
          <StatCard
            label="Tools Pinned"
            value={overview.tools_pinned ?? 0}
            trend={{ value: 8, positive: true }}
            sparklineData={overview.sparklines?.tools}
            sparklineColor="#3b82f6"
          />
          <StatCard
            label="Threats Blocked"
            value={overview.threats_blocked ?? 0}
            trend={{ value: 50, positive: false }}
            sparklineData={overview.sparklines?.threats}
            sparklineColor="#ef4444"
          />
          <div className="card" style={{ padding: 28 }}>
            <div className="metric-label">OWASP Coverage</div>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 20, marginTop: 12 }}>
              <ProgressRing
                value={overview.owasp_mcp_score ?? 0}
                max={overview.owasp_mcp_total ?? 10}
                size={64}
                strokeWidth={4}
                color="#8b5cf6"
                label="MCP"
              />
              <ProgressRing
                value={overview.owasp_agentic_score ?? 0}
                max={overview.owasp_agentic_total ?? 10}
                size={64}
                strokeWidth={4}
                color="#3b82f6"
                label="Agentic"
              />
            </div>
          </div>
        </div>
      </div>

      {/* Two-column: Feed + Trend */}
      <div ref={revealFeed} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'grid', gridTemplateColumns: '3fr 2fr', gap: 20 }}>
          {/* Live Feed */}
          <div>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
              <span className="section-label">Event Feed</span>
              <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                <span className="status-dot" />
                <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>Live</span>
              </div>
            </div>
            <div className="card" style={{ padding: 0, overflow: 'hidden' }}>
              <LiveFeed maxItems={14} />
            </div>
          </div>

          {/* Trend chart */}
          <div>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 12 }}>
              <span className="section-label">Threat Trend</span>
              <span style={{ fontSize: 11, fontFamily: 'var(--font-mono)', color: 'rgba(255,255,255,0.2)' }}>24h</span>
            </div>
            <div className="card" style={{ padding: '20px 16px 16px' }}>
              {trend && <TrendChart data={trend} height={200} />}
              <div style={{ display: 'flex', alignItems: 'center', gap: 16, marginTop: 12, paddingTop: 12, borderTop: '1px solid rgba(255,255,255,0.08)' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                  <span style={{ width: 6, height: 6, borderRadius: '50%', background: '#22c55e' }} />
                  <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>Clean</span>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                  <span style={{ width: 6, height: 6, borderRadius: '50%', background: '#f59e0b' }} />
                  <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>Warning</span>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                  <span style={{ width: 6, height: 6, borderRadius: '50%', background: '#ef4444' }} />
                  <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>Critical</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Servers table */}
      <div ref={revealServers} className="reveal" style={{ marginBottom: 48 }}>
        <span className="section-label" style={{ display: 'block', marginBottom: 12 }}>Servers</span>
        <div className="card" style={{ padding: '4px 0', overflow: 'hidden' }}>
          <table style={{ width: '100%', borderCollapse: 'collapse' }}>
            <thead>
              <tr>
                <th style={{ textAlign: 'left', padding: '10px 20px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)' }}>Status</th>
                <th style={{ textAlign: 'left', padding: '10px 20px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)' }}>Name</th>
                <th style={{ textAlign: 'right', padding: '10px 20px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)' }}>Tools</th>
                <th style={{ textAlign: 'left', padding: '10px 20px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)' }}>Trust</th>
                <th style={{ textAlign: 'right', padding: '10px 20px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)' }}>Last Scan</th>
              </tr>
            </thead>
            <tbody>
              {recentServers.map((server) => (
                <tr key={server.name} style={{ transition: 'background 0.1s ease' }} onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.02)'} onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}>
                  <td style={{ padding: '10px 20px', borderBottom: '1px solid rgba(255,255,255,0.02)' }}>
                    <span style={{
                      display: 'inline-block',
                      width: 6, height: 6,
                      borderRadius: '50%',
                      background: server.status === 'active' ? '#22c55e' : '#ef4444',
                    }} />
                  </td>
                  <td style={{ padding: '10px 20px', borderBottom: '1px solid rgba(255,255,255,0.02)' }}>
                    <span style={{ fontSize: 13, color: 'rgba(255,255,255,0.9)' }}>{server.name}</span>
                    {server.status === 'blocked' && (
                      <SeverityBadge severity="critical" className="ml-2" />
                    )}
                  </td>
                  <td style={{ padding: '10px 20px', textAlign: 'right', borderBottom: '1px solid rgba(255,255,255,0.02)' }}>
                    <span style={{ fontFamily: 'var(--font-mono)', fontSize: 13, fontVariantNumeric: 'tabular-nums', color: 'rgba(255,255,255,0.5)' }}>{server.tools}</span>
                  </td>
                  <td style={{ padding: '10px 20px', borderBottom: '1px solid rgba(255,255,255,0.02)' }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                      <div className="trust-bar">
                        <div
                          className={`trust-bar-fill ${server.trust >= 80 ? 'high' : server.trust >= 50 ? 'medium' : 'low'}`}
                          style={{ width: `${server.trust}%`, transition: 'width 0.8s cubic-bezier(0.16, 1, 0.3, 1)' }}
                        />
                      </div>
                      <span className={trustScoreColor(server.trust)} style={{ fontFamily: 'var(--font-mono)', fontSize: 12, fontVariantNumeric: 'tabular-nums' }}>
                        {server.trust}
                      </span>
                    </div>
                  </td>
                  <td style={{ padding: '10px 20px', textAlign: 'right', borderBottom: '1px solid rgba(255,255,255,0.02)' }}>
                    <span style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontVariantNumeric: 'tabular-nums' }}>{server.lastScan}</span>
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>

      {/* Configs */}
      <div ref={revealConfigs} className="reveal">
        <span className="section-label" style={{ display: 'block', marginBottom: 12 }}>Discovered Configurations</span>
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 12 }}>
          {(overview.configs_found ?? []).map((config) => (
            <div
              key={config.name}
              className="card"
              style={{ padding: '16px 20px' }}
            >
              <div style={{ fontSize: 13, fontWeight: 450, color: 'rgba(255,255,255,0.9)' }}>{config.name}</div>
              <div style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', marginTop: 4, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{config.path}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  )
}
