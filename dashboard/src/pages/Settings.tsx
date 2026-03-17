import { useApi } from '../hooks/useApi'
import { getConfig } from '../api/client'
import { useReveal } from '../hooks/useReveal'

export function Settings() {
  const { data: config } = useApi(getConfig)
  const revealHeader = useReveal()
  const revealContent = useReveal()

  if (!config) {
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

  const sections = [
    { title: 'Global', data: config.global },
    { title: 'Scan', data: config.scan },
    { title: 'Proxy', data: config.proxy },
    { title: 'LLM', data: config.llm },
    { title: 'Servers', data: config.servers },
  ]

  return (
    <div>
      <div ref={revealHeader} className="reveal" style={{ marginBottom: 48 }}>
        <h1 className="page-title">Settings</h1>
        <p className="page-subtitle">Current MCPShield configuration (read-only)</p>
      </div>

      <div ref={revealContent} className="reveal" style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
        {sections.map((section) => (
          <div key={section.title} className="card" style={{ padding: 24 }}>
            <span className="section-label" style={{ display: 'block', marginBottom: 16 }}>{section.title}</span>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
              {Object.entries(section.data as Record<string, unknown>).map(([key, value]) => (
                <ConfigRow key={key} keyName={key} value={value} />
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}

function ConfigRow({ keyName, value }: { keyName: string; value: unknown }) {
  if (typeof value === 'object' && value !== null) {
    return (
      <div style={{ paddingLeft: 12, borderLeft: '1px solid rgba(255,255,255,0.08)', marginLeft: 4, marginTop: 8, marginBottom: 8 }}>
        <span style={{ fontSize: 10, fontWeight: 500, textTransform: 'uppercase' as const, letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)' }}>{keyName}</span>
        <div style={{ marginTop: 4, display: 'flex', flexDirection: 'column', gap: 2 }}>
          {Object.entries(value as Record<string, unknown>).map(([k, v]) => (
            <ConfigRow key={k} keyName={k} value={v} />
          ))}
        </div>
      </div>
    )
  }

  const isStatus = keyName === 'status'
  const isConnected = isStatus && value === 'connected'

  return (
    <div
      style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        padding: '6px 12px',
        borderRadius: 8,
        transition: 'background 0.1s ease',
      }}
      onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.02)'}
      onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
    >
      <span style={{ fontSize: 13, color: 'rgba(255,255,255,0.5)' }}>{keyName}</span>
      <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
        {isStatus && (
          <span style={{ width: 6, height: 6, borderRadius: '50%', background: isConnected ? '#22c55e' : '#ef4444' }} />
        )}
        <span style={{
          fontFamily: 'var(--font-mono)',
          fontSize: 13,
          fontVariantNumeric: 'tabular-nums',
          color: typeof value === 'boolean'
            ? value ? '#22c55e' : '#ef4444'
            : 'rgba(255,255,255,0.9)',
        }}>
          {String(value)}
        </span>
      </div>
    </div>
  )
}
