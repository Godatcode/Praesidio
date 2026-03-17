import { useApi } from '../hooks/useApi'
import { getComplianceMCP, getComplianceAgentic } from '../api/client'
import { ProgressRing } from '../components/ProgressRing'
import { OWASPScorecard } from '../components/OWASPScorecard'
import { useReveal } from '../hooks/useReveal'

export function Compliance() {
  const { data: mcp } = useApi(getComplianceMCP)
  const { data: agentic } = useApi(getComplianceAgentic)
  const revealHeader = useReveal()
  const revealRings = useReveal()
  const revealCards = useReveal()
  const revealLegend = useReveal()

  const mcpCovered = mcp?.filter((i) => i.status === 'covered').length ?? 0
  const mcpTotal = mcp?.length ?? 10
  const agenticCovered = agentic?.filter((i) => i.status === 'covered').length ?? 0
  const agenticTotal = agentic?.length ?? 10

  return (
    <div>
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'flex', alignItems: 'flex-start', justifyContent: 'space-between' }}>
          <div>
            <h1 className="page-title">Compliance</h1>
            <p className="page-subtitle">OWASP coverage across MCP and Agentic security frameworks</p>
          </div>
          <button className="btn" style={{ marginTop: 12 }}>Export Report</button>
        </div>
      </div>

      <div ref={revealRings} className="reveal" style={{ marginBottom: 48 }}>
        <div className="card" style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', gap: 64, padding: '40px 0' }}>
          <div style={{ textAlign: 'center' }}>
            <ProgressRing value={mcpCovered} max={mcpTotal} size={100} strokeWidth={5} color="#8b5cf6" />
            <p style={{ fontSize: 13, fontWeight: 450, color: 'rgba(255,255,255,0.5)', marginTop: 12 }}>OWASP MCP Top 10</p>
          </div>
          <div style={{ width: 1, height: 80, background: 'rgba(255,255,255,0.08)' }} />
          <div style={{ textAlign: 'center' }}>
            <ProgressRing value={agenticCovered} max={agenticTotal} size={100} strokeWidth={5} color="#3b82f6" />
            <p style={{ fontSize: 13, fontWeight: 450, color: 'rgba(255,255,255,0.5)', marginTop: 12 }}>OWASP Agentic Top 10</p>
          </div>
        </div>
      </div>

      <div ref={revealCards} className="reveal" style={{ marginBottom: 48 }}>
        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 20 }}>
          <div className="card" style={{ padding: 24 }}>
            {mcp && <OWASPScorecard title="OWASP MCP Top 10" items={mcp} />}
          </div>
          <div className="card" style={{ padding: 24 }}>
            {agentic && <OWASPScorecard title="OWASP Agentic Top 10" items={agentic} />}
          </div>
        </div>
      </div>

      <div ref={revealLegend} className="reveal" style={{ display: 'flex', alignItems: 'center', gap: 24, fontSize: 11, color: 'rgba(255,255,255,0.3)' }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
          <span style={{ width: 6, height: 6, borderRadius: '50%', background: '#22c55e' }} />
          Covered
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
          <span style={{ width: 6, height: 6, borderRadius: '50%', background: '#f59e0b' }} />
          Partial
        </div>
        <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
          <span style={{ width: 6, height: 6, borderRadius: '50%', background: '#ef4444' }} />
          Missing
        </div>
      </div>
    </div>
  )
}
