interface SeverityBadgeProps {
  severity: string
  className?: string
}

function badgeClass(severity: string): string {
  switch (severity) {
    case 'critical': return 'badge-critical'
    case 'high': return 'badge-high'
    case 'warning': return 'badge-warning'
    case 'medium': return 'badge-medium'
    case 'low': return 'badge-low'
    case 'info': return 'badge-info'
    case 'clean': return 'badge-clean'
    default: return ''
  }
}

export function SeverityBadge({ severity, className }: SeverityBadgeProps) {
  return (
    <span className={`badge ${badgeClass(severity)} ${className ?? ''}`}>
      {severity}
    </span>
  )
}
