import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  CartesianGrid,
  Tooltip,
  ResponsiveContainer,
} from 'recharts'

interface TrendChartProps {
  data: { hour: string; clean: number; warning: number; critical: number }[]
  height?: number
}

const tooltipStyle = {
  backgroundColor: 'rgba(17, 17, 19, 0.95)',
  border: '1px solid rgba(255,255,255,0.08)',
  borderRadius: '10px',
  fontSize: '11px',
  fontFamily: 'JetBrains Mono',
  color: 'rgba(255,255,255,0.9)',
  boxShadow: '0 8px 32px rgba(0,0,0,0.5)',
}

export function TrendChart({ data, height = 220 }: TrendChartProps) {
  return (
    <ResponsiveContainer width="100%" height={height}>
      <AreaChart data={data} margin={{ top: 4, right: 4, bottom: 0, left: -20 }}>
        <defs>
          <linearGradient id="gradClean" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="#22c55e" stopOpacity={0.3} />
            <stop offset="100%" stopColor="#22c55e" stopOpacity={0.02} />
          </linearGradient>
          <linearGradient id="gradWarning" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="#f59e0b" stopOpacity={0.3} />
            <stop offset="100%" stopColor="#f59e0b" stopOpacity={0.02} />
          </linearGradient>
          <linearGradient id="gradCritical" x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor="#ef4444" stopOpacity={0.3} />
            <stop offset="100%" stopColor="#ef4444" stopOpacity={0.02} />
          </linearGradient>
        </defs>
        <CartesianGrid strokeDasharray="3 3" stroke="rgba(255,255,255,0.08)" vertical={false} />
        <XAxis
          dataKey="hour"
          tick={{ fill: 'rgba(255,255,255,0.3)', fontSize: 10, fontFamily: 'JetBrains Mono' }}
          tickLine={false}
          axisLine={false}
          interval={5}
        />
        <YAxis
          tick={{ fill: 'rgba(255,255,255,0.3)', fontSize: 10, fontFamily: 'JetBrains Mono' }}
          tickLine={false}
          axisLine={false}
        />
        <Tooltip
          contentStyle={tooltipStyle}
          itemStyle={{ color: 'rgba(255,255,255,0.5)' }}
          labelStyle={{ color: 'rgba(255,255,255,0.9)', marginBottom: '4px' }}
        />
        <Area
          type="monotone"
          dataKey="clean"
          stroke="#22c55e"
          strokeWidth={1.5}
          fill="url(#gradClean)"
          dot={false}
          activeDot={{ fill: '#22c55e', r: 4, strokeWidth: 0 }}
          name="Clean"
          isAnimationActive={true}
          animationDuration={1200}
        />
        <Area
          type="monotone"
          dataKey="warning"
          stroke="#f59e0b"
          strokeWidth={1.5}
          fill="url(#gradWarning)"
          dot={false}
          activeDot={{ fill: '#f59e0b', r: 4, strokeWidth: 0 }}
          name="Warning"
          isAnimationActive={true}
          animationDuration={1200}
          animationBegin={200}
        />
        <Area
          type="monotone"
          dataKey="critical"
          stroke="#ef4444"
          strokeWidth={1.5}
          fill="url(#gradCritical)"
          dot={false}
          activeDot={{ fill: '#ef4444', r: 4, strokeWidth: 0 }}
          name="Critical"
          isAnimationActive={true}
          animationDuration={1200}
          animationBegin={400}
        />
      </AreaChart>
    </ResponsiveContainer>
  )
}
