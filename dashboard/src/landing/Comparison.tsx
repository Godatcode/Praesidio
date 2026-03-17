import { useLandingReveal } from '../hooks/useLandingReveal'
import { Check, X } from 'lucide-react'

const products = ['Praesidio', 'mcp-scan', 'Onyx', 'Strata']
const features = [
  { name: 'Runtime proxy / firewall', values: [true, false, false, true] },
  { name: 'Tool description scanning', values: [true, true, false, false] },
  { name: 'LLM-powered analysis', values: [true, false, false, false] },
  { name: 'Hash pinning (supply chain)', values: [true, true, false, false] },
  { name: 'Behavioral anomaly detection', values: [true, false, false, false] },
  { name: 'Output credential filtering', values: [true, false, false, true] },
  { name: 'Honeypot decoy tools', values: [true, false, false, false] },
  { name: 'OWASP MCP Top 10 coverage', values: [true, false, false, false] },
  { name: 'Cross-server shadow detection', values: [true, false, false, false] },
  { name: 'Open source', values: [true, true, false, false] },
]

export function Comparison() {
  const titleRef = useLandingReveal({ y: 20 })
  const tableRef = useLandingReveal({ y: 30, delay: 0.1 })

  return (
    <section id="comparison" style={{ padding: '120px 40px', maxWidth: 1100, margin: '0 auto' }}>
      <div ref={titleRef} style={{ textAlign: 'center', marginBottom: 64 }}>
        <h2
          style={{
            fontFamily: "'Clash Display', sans-serif",
            fontSize: 42,
            fontWeight: 500,
            letterSpacing: '-0.03em',
            color: '#f0f0f2',
            lineHeight: 1.15,
          }}
        >
          Why teams choose Praesidio
        </h2>
      </div>

      <div
        ref={tableRef}
        style={{
          background: 'rgba(255,255,255,0.05)',
          border: '1px solid rgba(255,255,255,0.08)',
          borderRadius: 16,
          overflow: 'hidden',
        }}
      >
        {/* Header */}
        <div
          style={{
            display: 'grid',
            gridTemplateColumns: '2fr repeat(4, 1fr)',
            borderBottom: '1px solid rgba(255,255,255,0.06)',
          }}
        >
          <div style={{ padding: '16px 24px' }} />
          {products.map((p, i) => (
            <div
              key={p}
              style={{
                padding: '16px 24px',
                textAlign: 'center',
                fontFamily: "'Clash Display', sans-serif",
                fontSize: 14,
                fontWeight: 500,
                color: i === 0 ? '#7c5bf0' : 'rgba(255,255,255,0.4)',
                background: i === 0 ? 'rgba(124, 91, 240, 0.04)' : 'transparent',
                letterSpacing: '-0.02em',
              }}
            >
              {p}
            </div>
          ))}
        </div>

        {/* Rows */}
        {features.map((feature, rowIdx) => (
          <div
            key={feature.name}
            style={{
              display: 'grid',
              gridTemplateColumns: '2fr repeat(4, 1fr)',
              borderBottom: rowIdx < features.length - 1 ? '1px solid rgba(255,255,255,0.04)' : 'none',
              transition: 'background 0.15s ease',
            }}
            onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.04)'}
            onMouseLeave={(e) => e.currentTarget.style.background = 'transparent'}
          >
            <div style={{ padding: '12px 24px', fontSize: 13, color: 'rgba(255,255,255,0.6)' }}>
              {feature.name}
            </div>
            {feature.values.map((v, i) => (
              <div
                key={i}
                style={{
                  padding: '12px 24px',
                  textAlign: 'center',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  background: i === 0 ? 'rgba(124, 91, 240, 0.04)' : 'transparent',
                }}
              >
                {v ? (
                  <Check size={16} color="#22c55e" strokeWidth={2.5} />
                ) : (
                  <X size={16} color="rgba(255,255,255,0.15)" strokeWidth={2} />
                )}
              </div>
            ))}
          </div>
        ))}
      </div>
    </section>
  )
}
