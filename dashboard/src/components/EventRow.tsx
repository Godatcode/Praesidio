import { useRelativeTime } from '../hooks/useRelativeTime'
import type { AuditEvent } from '../api/mock'

interface EventRowProps {
  event: AuditEvent
  index?: number
}

function severityDotClass(severity: string): string {
  switch (severity) {
    case 'critical': return 'critical'
    case 'high': return 'high'
    case 'warning': return 'warning'
    case 'medium': return 'medium'
    case 'low': return 'low'
    case 'info': return 'info'
    case 'clean': return 'clean'
    default: return ''
  }
}

export function EventRow({ event, index = 0 }: EventRowProps) {
  const timeAgo = useRelativeTime(event.timestamp)

  return (
    <div
      className="feed-row reveal"
      style={{ transitionDelay: `${index * 30}ms` }}
    >
      <span className="feed-time">{timeAgo}</span>
      <span className={`severity-dot ${severityDotClass(event.severity)}`} />
      <span className="feed-target">
        {event.tool ? `${event.server}.${event.tool}` : event.server}
      </span>
      <span className="feed-desc">{event.description}</span>
    </div>
  )
}
