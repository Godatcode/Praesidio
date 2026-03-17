import { AnimatedNumber } from './AnimatedNumber'
import { Sparkline } from './Sparkline'

interface StatCardProps {
  label: string
  value: number
  trend?: { value: number; positive?: boolean }
  sparklineData?: number[]
  sparklineColor?: string
}

export function StatCard({
  label,
  value,
  trend,
  sparklineData,
  sparklineColor = '#8b5cf6',
}: StatCardProps) {
  return (
    <div className="card metric-card">
      <div className="metric-label">{label}</div>
      <div style={{ display: 'flex', alignItems: 'baseline' }}>
        <AnimatedNumber value={value} className="metric-value" />
        {trend && (
          <span className={`metric-trend ${trend.positive !== false ? 'up' : 'down'}`}>
            {trend.positive !== false ? '+' : '-'}{Math.abs(trend.value)}%
          </span>
        )}
      </div>

      {sparklineData && sparklineData.length > 0 && (
        <div className="metric-sparkline">
          <Sparkline data={sparklineData} color={sparklineColor} height={40} />
        </div>
      )}
    </div>
  )
}
