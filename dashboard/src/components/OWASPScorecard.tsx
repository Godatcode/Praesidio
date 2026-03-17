import type { ComplianceItem } from '../api/mock'

interface OWASPScorecardProps {
  title: string
  items: ComplianceItem[]
  className?: string
}

export function OWASPScorecard({ title, items, className }: OWASPScorecardProps) {
  return (
    <div className={className} style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
      <h3 className="section-label">{title}</h3>
      <div style={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
        {items.map((item) => (
          <div
            key={item.id}
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
            <StatusDot status={item.status} />
            <span style={{
              fontFamily: 'var(--font-mono)',
              fontSize: 11,
              color: 'rgba(255,255,255,0.3)',
              fontVariantNumeric: 'tabular-nums',
              width: 56,
              flexShrink: 0,
            }}>
              {item.id}
            </span>
            <span style={{
              fontSize: 13,
              color: 'rgba(255,255,255,0.9)',
              flex: 1,
              minWidth: 0,
              overflow: 'hidden',
              textOverflow: 'ellipsis',
              whiteSpace: 'nowrap',
            }}>
              {item.name}
            </span>
            <span style={{
              fontFamily: 'var(--font-mono)',
              fontSize: 11,
              color: 'rgba(255,255,255,0.2)',
              flexShrink: 0,
            }}>
              {item.module}
            </span>
          </div>
        ))}
      </div>
    </div>
  )
}

function StatusDot({ status }: { status: 'covered' | 'partial' | 'missing' }) {
  const color =
    status === 'covered' ? '#22c55e' :
    status === 'partial' ? '#f59e0b' :
    '#ef4444'

  return (
    <span
      style={{
        width: 6,
        height: 6,
        borderRadius: '50%',
        background: color,
        flexShrink: 0,
      }}
      title={status}
    />
  )
}
