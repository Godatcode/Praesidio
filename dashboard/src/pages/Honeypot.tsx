import { useApi } from '../hooks/useApi'
import { getHoneypotAttacks, getHoneypotStatus } from '../api/client'
import { SeverityBadge } from '../components/SeverityBadge'
import { useRelativeTime } from '../hooks/useRelativeTime'
import { useReveal } from '../hooks/useReveal'
import { PieChart, Pie, Cell, ResponsiveContainer, LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip } from 'recharts'

const tooltipStyle = {
  backgroundColor: 'rgba(17, 17, 19, 0.95)',
  border: '1px solid rgba(255,255,255,0.08)',
  borderRadius: '10px',
  fontSize: '11px',
  fontFamily: 'JetBrains Mono',
  color: 'rgba(255,255,255,0.9)',
  boxShadow: '0 8px 32px rgba(0,0,0,0.5)',
}

export function Honeypot() {
  const { data: attacks } = useApi(getHoneypotAttacks)
  const { data: status } = useApi(getHoneypotStatus)
  const revealHeader = useReveal()
  const revealStatus = useReveal()
  const revealCharts = useReveal()
  const revealFeed = useReveal()

  const typeCounts: Record<string, number> = {}
  const attackList = Array.isArray(attacks) ? attacks : []
  attackList.forEach((a) => {
    typeCounts[a.attack_type] = (typeCounts[a.attack_type] ?? 0) + 1
  })
  const pieData = Object.entries(typeCounts).map(([name, value]) => ({ name, value }))

  const PIE_COLORS = ['#ef4444', '#f59e0b', '#3b82f6', '#8b5cf6', '#22c55e', '#f97316']

  const timeData = Array.from({ length: 12 }, (_, i) => ({
    hour: `${String((24 - 12 + i) % 24).padStart(2, '0')}:00`,
    attacks: Math.floor(Math.random() * 5) + (i > 8 ? 2 : 0),
  }))

  return (
    <div>
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <h1 className="page-title">Honeypot</h1>
        <p className="page-subtitle">Decoy tools that detect and log malicious interactions</p>
      </div>

      <div ref={revealStatus} className="reveal" style={{ marginBottom: 48 }}>
        <div className="card" style={{ padding: '20px 24px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
          <div style={{ display: 'flex', alignItems: 'center', gap: 16 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
              <span style={{
                width: 8, height: 8, borderRadius: '50%',
                background: status?.running ? '#22c55e' : 'rgba(255,255,255,0.15)',
              }} />
              <span style={{ fontSize: 14, fontWeight: 450, color: 'rgba(255,255,255,0.9)' }}>
                Honeypot: {status?.running ? 'Running' : 'Stopped'}
              </span>
            </div>
            {status && (
              <div style={{ display: 'flex', alignItems: 'center', gap: 16, marginLeft: 16, fontSize: 12, color: 'rgba(255,255,255,0.3)' }}>
                <span>Uptime: <span style={{ fontFamily: 'var(--font-mono)', fontVariantNumeric: 'tabular-nums', color: 'rgba(255,255,255,0.5)' }}>{status.uptime_hours}h</span></span>
                <span>Total attacks: <span style={{ fontFamily: 'var(--font-mono)', fontVariantNumeric: 'tabular-nums', color: 'rgba(255,255,255,0.5)' }}>{status.total_attacks}</span></span>
              </div>
            )}
          </div>
          <button className={status?.running ? 'btn btn-danger' : 'btn btn-primary'}>
            {status?.running ? 'Stop' : 'Start'}
          </button>
        </div>
      </div>

      <div ref={revealCharts} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'grid', gridTemplateColumns: '2fr 3fr', gap: 20 }}>
          <div>
            <span className="section-label" style={{ display: 'block', marginBottom: 12 }}>Attack Types</span>
            <div className="card" style={{ padding: 20 }}>
              <ResponsiveContainer width="100%" height={200}>
                <PieChart>
                  <Pie data={pieData} cx="50%" cy="50%" innerRadius={50} outerRadius={80} paddingAngle={3} dataKey="value" stroke="none" isAnimationActive animationDuration={1000}>
                    {pieData.map((_, i) => <Cell key={i} fill={PIE_COLORS[i % PIE_COLORS.length]} />)}
                  </Pie>
                  <Tooltip contentStyle={tooltipStyle} />
                </PieChart>
              </ResponsiveContainer>
              <div style={{ display: 'flex', flexWrap: 'wrap', gap: 12, marginTop: 8, justifyContent: 'center' }}>
                {pieData.map((entry, i) => (
                  <div key={entry.name} style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                    <span style={{ width: 6, height: 6, borderRadius: '50%', background: PIE_COLORS[i % PIE_COLORS.length] }} />
                    <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>{entry.name.replace(/_/g, ' ')}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>

          <div>
            <span className="section-label" style={{ display: 'block', marginBottom: 12 }}>Attacks Over Time</span>
            <div className="card" style={{ padding: 20 }}>
              <ResponsiveContainer width="100%" height={200}>
                <LineChart data={timeData} margin={{ top: 4, right: 4, bottom: 0, left: -20 }}>
                  <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.08)" vertical={false} />
                  <XAxis dataKey="hour" tick={{ fill: 'rgba(255,255,255,0.3)', fontSize: 10, fontFamily: 'JetBrains Mono' }} tickLine={false} axisLine={false} interval={2} />
                  <YAxis tick={{ fill: 'rgba(255,255,255,0.3)', fontSize: 10, fontFamily: 'JetBrains Mono' }} tickLine={false} axisLine={false} />
                  <Tooltip contentStyle={tooltipStyle} />
                  <Line type="monotone" dataKey="attacks" stroke="#ef4444" strokeWidth={1.5} dot={{ fill: '#ef4444', r: 2, strokeWidth: 0 }} activeDot={{ fill: '#ef4444', r: 4, strokeWidth: 0 }} isAnimationActive animationDuration={1200} />
                </LineChart>
              </ResponsiveContainer>
            </div>
          </div>
        </div>
      </div>

      <div ref={revealFeed} className="reveal">
        <span className="section-label" style={{ display: 'block', marginBottom: 12 }}>Attack Feed</span>
        <div className="card" style={{ padding: '4px 0', overflow: 'hidden' }}>
          <table style={{ width: '100%', borderCollapse: 'collapse' }}>
            <thead>
              <tr>
                {['Time', 'Source', 'Tool Called', 'Attack Type', 'Severity', 'Input Preview'].map((h) => (
                  <th key={h} style={{ textAlign: 'left', padding: '10px 16px', fontSize: 11, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', borderBottom: '1px solid rgba(255,255,255,0.08)' }}>{h}</th>
                ))}
              </tr>
            </thead>
            <tbody>
              {attackList.map((attack) => (
                <AttackRow key={attack.id} attack={attack} />
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  )
}

function AttackRow({ attack }: {
  attack: { id: string; timestamp: string; source: string; tool_called: string; attack_type: string; input_preview: string; severity: string }
}) {
  const timeAgo = useRelativeTime(attack.timestamp)
  const tdStyle: React.CSSProperties = { padding: '10px 16px', borderBottom: '1px solid rgba(255,255,255,0.02)' }

  return (
    <tr style={{ transition: 'background 0.1s ease' }} onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.02)'} onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}>
      <td style={tdStyle}><span style={{ fontFamily: 'var(--font-mono)', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontVariantNumeric: 'tabular-nums' }}>{timeAgo}</span></td>
      <td style={tdStyle}><span style={{ fontSize: 12, color: 'rgba(255,255,255,0.5)' }}>{attack.source}</span></td>
      <td style={tdStyle}><span style={{ fontFamily: 'var(--font-mono)', fontSize: 12, color: 'rgba(255,255,255,0.9)' }}>{attack.tool_called}</span></td>
      <td style={tdStyle}><span style={{ fontSize: 11, color: 'rgba(255,255,255,0.4)', background: 'rgba(255,255,255,0.06)', padding: '2px 8px', borderRadius: 4 }}>{attack.attack_type.replace(/_/g, ' ')}</span></td>
      <td style={tdStyle}><SeverityBadge severity={attack.severity} /></td>
      <td style={tdStyle}><code style={{ fontFamily: 'var(--font-mono)', fontSize: 10, color: 'rgba(255,255,255,0.2)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', display: 'block', maxWidth: 300 }}>{attack.input_preview}</code></td>
    </tr>
  )
}
