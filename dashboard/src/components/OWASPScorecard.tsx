import React, { useEffect, useState } from 'react'

interface Compliance {
  mcp_top10_coverage: number
  agentic_top10_coverage: number
  total_risks: number
  covered: number
}

export function OWASPScorecard() {
  const [data, setData] = useState<Compliance | null>(null)

  useEffect(() => {
    fetch('/api/compliance')
      .then(res => res.json())
      .then(setData)
      .catch(console.error)
  }, [])

  if (!data) return <div style={{ color: '#666' }}>Loading...</div>

  const percentage = Math.round((data.covered / data.total_risks) * 100)

  return (
    <div style={{ border: '1px solid #dee2e6', borderRadius: 8, padding: 16 }}>
      <div style={{ textAlign: 'center', marginBottom: 16 }}>
        <div style={{ fontSize: 48, fontWeight: 700, color: percentage === 100 ? '#28a745' : '#ffc107' }}>
          {percentage}%
        </div>
        <div style={{ color: '#666' }}>OWASP Coverage</div>
      </div>

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 12 }}>
        <div style={{ background: '#f8f9fa', padding: 12, borderRadius: 6, textAlign: 'center' }}>
          <div style={{ fontWeight: 600 }}>MCP Top 10</div>
          <div style={{ fontSize: 24, color: '#28a745' }}>{data.mcp_top10_coverage}/10</div>
        </div>
        <div style={{ background: '#f8f9fa', padding: 12, borderRadius: 6, textAlign: 'center' }}>
          <div style={{ fontWeight: 600 }}>Agentic Top 10</div>
          <div style={{ fontSize: 24, color: '#28a745' }}>{data.agentic_top10_coverage}/10</div>
        </div>
      </div>
    </div>
  )
}
