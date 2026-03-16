import React, { useEffect, useState } from 'react'
import { LiveFeed } from './components/LiveFeed'
import { OWASPScorecard } from './components/OWASPScorecard'

interface Overview {
  servers: number
  tools_pinned: number
  configs_found: { name: string; path: string }[]
}

function App() {
  const [overview, setOverview] = useState<Overview | null>(null)

  useEffect(() => {
    fetch('/api/overview')
      .then(res => res.json())
      .then(setOverview)
      .catch(console.error)
  }, [])

  return (
    <div style={{ fontFamily: 'system-ui', maxWidth: 1200, margin: '0 auto', padding: 20 }}>
      <header style={{ borderBottom: '2px solid #333', paddingBottom: 16, marginBottom: 24 }}>
        <h1 style={{ fontSize: 28, fontWeight: 700 }}>
          🛡️ MCPShield Dashboard
        </h1>
        <p style={{ color: '#666', margin: 0 }}>AI Agent Security Platform</p>
      </header>

      {overview && (
        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 16, marginBottom: 24 }}>
          <StatCard label="Configs Found" value={overview.configs_found.length} />
          <StatCard label="Servers" value={overview.servers} />
          <StatCard label="Tools Pinned" value={overview.tools_pinned} />
        </div>
      )}

      <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 24 }}>
        <div>
          <h2 style={{ fontSize: 20, marginBottom: 12 }}>Live Event Feed</h2>
          <LiveFeed />
        </div>
        <div>
          <h2 style={{ fontSize: 20, marginBottom: 12 }}>OWASP Compliance</h2>
          <OWASPScorecard />
        </div>
      </div>
    </div>
  )
}

function StatCard({ label, value }: { label: string; value: number }) {
  return (
    <div style={{
      background: '#f8f9fa',
      border: '1px solid #dee2e6',
      borderRadius: 8,
      padding: 16,
      textAlign: 'center',
    }}>
      <div style={{ fontSize: 32, fontWeight: 700 }}>{value}</div>
      <div style={{ color: '#666', fontSize: 14 }}>{label}</div>
    </div>
  )
}

export default App
