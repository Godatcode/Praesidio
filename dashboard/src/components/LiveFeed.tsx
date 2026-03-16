import React, { useEffect, useState } from 'react'

interface AuditEvent {
  timestamp: string
  event_type: string
  severity: string
  server: string
  tool?: string
  description: string
}

const severityIcon: Record<string, string> = {
  critical: '🚨',
  high: '🔴',
  medium: '⚠️',
  low: '🔵',
  info: 'ℹ️',
}

export function LiveFeed() {
  const [events, setEvents] = useState<AuditEvent[]>([])

  useEffect(() => {
    fetch('/api/events')
      .then(res => res.json())
      .then(setEvents)
      .catch(console.error)

    const interval = setInterval(() => {
      fetch('/api/events')
        .then(res => res.json())
        .then(setEvents)
        .catch(console.error)
    }, 5000)

    return () => clearInterval(interval)
  }, [])

  if (events.length === 0) {
    return <div style={{ color: '#666', padding: 16 }}>No events yet. Run a scan to generate events.</div>
  }

  return (
    <div style={{ maxHeight: 400, overflow: 'auto', border: '1px solid #dee2e6', borderRadius: 8 }}>
      {events.slice(0, 50).map((event, i) => (
        <div key={i} style={{
          padding: '8px 12px',
          borderBottom: '1px solid #eee',
          fontSize: 13,
          fontFamily: 'monospace',
        }}>
          <span>{severityIcon[event.severity] || '•'} </span>
          <span style={{ color: '#666' }}>
            {new Date(event.timestamp).toLocaleTimeString()}
          </span>
          {' '}
          <strong>{event.server}</strong>
          {event.tool && <span>.{event.tool}</span>}
          {' — '}
          <span>{event.description}</span>
        </div>
      ))}
    </div>
  )
}
