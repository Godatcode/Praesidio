import { useEffect } from 'react'
import { usePollingApi } from '../hooks/useApi'
import { getEvents } from '../api/client'
import { EventRow } from './EventRow'
import { useRevealChildren } from '../hooks/useReveal'

interface LiveFeedProps {
  maxItems?: number
}

export function LiveFeed({ maxItems = 20 }: LiveFeedProps) {
  const { data: events } = usePollingApi(getEvents, 5000)
  const containerRef = useRevealChildren<HTMLDivElement>()

  const items = (events ?? []).slice(0, maxItems)

  // Re-observe when items change
  useEffect(() => {
    const el = containerRef.current
    if (!el) return
    const observer = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('revealed')
            observer.unobserve(entry.target)
          }
        })
      },
      { threshold: 0.1 },
    )
    el.querySelectorAll('.reveal:not(.revealed)').forEach((child) => observer.observe(child))
    return () => observer.disconnect()
  }, [items.length, containerRef])

  return (
    <div ref={containerRef} className="overflow-y-auto max-h-[420px]" style={{ borderRadius: 10 }}>
      {items.length === 0 && (
        <div style={{ textAlign: 'center', padding: '32px 0', color: 'rgba(255,255,255,0.2)', fontSize: 13 }}>
          No events recorded
        </div>
      )}
      {items.map((event, i) => (
        <EventRow key={event.id} event={event} index={i} />
      ))}
    </div>
  )
}
