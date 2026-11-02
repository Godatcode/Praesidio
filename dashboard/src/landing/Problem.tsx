import { useState, useEffect, useRef } from 'react'
import { useLandingReveal } from '../hooks/useLandingReveal'
import { useMediaQuery, MOBILE_QUERY } from '../hooks/useMediaQuery'

function CountUp({ target, suffix = '' }: { target: number; suffix?: string }) {
  const [count, setCount] = useState(0)
  const ref = useRef<HTMLSpanElement>(null)
  const started = useRef(false)

  useEffect(() => {
    const el = ref.current
    if (!el) return
    const obs = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting && !started.current) {
          started.current = true
          const duration = 1500
          const start = performance.now()
          function tick(now: number) {
            const p = Math.min((now - start) / duration, 1)
            const eased = 1 - Math.pow(1 - p, 3)
            setCount(Math.round(eased * target))
            if (p < 1) requestAnimationFrame(tick)
          }
          requestAnimationFrame(tick)
          obs.disconnect()
        }
      },
      { threshold: 0.5 },
    )
    obs.observe(el)
    return () => obs.disconnect()
  }, [target])

  return <span ref={ref}>{count.toLocaleString()}{suffix}</span>
}

function StatCard({ target, suffix, label, delay, isMobile }: { target: number; suffix: string; label: string; delay: number; isMobile: boolean }) {
  const cardRef = useLandingReveal({ y: 30, delay })

  return (
    <div
      ref={cardRef}
      className="stat-card"
      style={{
        flex: isMobile ? undefined : 1,
        minWidth: 0,
        textAlign: 'center',
        padding: isMobile ? '32px 24px' : '48px 20px',
        background: 'rgba(255,255,255,0.05)',
        border: '1px solid rgba(255,255,255,0.08)',
        borderRadius: 16,
        cursor: 'default',
      }}
    >
      <span
        style={{
          fontFamily: "'Clash Display', sans-serif",
          fontSize: isMobile ? 48 : 'clamp(36px, 4.6vw, 56px)',
          fontWeight: 600,
          letterSpacing: '-0.03em',
          background: 'linear-gradient(180deg, #f0f0f2 0%, rgba(240,240,242,0.5) 100%)',
          WebkitBackgroundClip: 'text',
          WebkitTextFillColor: 'transparent',
          backgroundClip: 'text',
          display: 'block',
          lineHeight: 1,
        }}
      >
        <CountUp target={target} suffix={suffix} />
      </span>
      <span style={{ fontSize: 14, color: 'rgba(255,255,255,0.5)', marginTop: 12, display: 'block' }}>{label}</span>
    </div>
  )
}

export function Problem() {
  const titleRef = useLandingReveal({ y: 20 })
  const isMobile = useMediaQuery(MOBILE_QUERY)

  return (
    <section style={{ padding: isMobile ? '80px 16px' : '120px 40px', maxWidth: 1100, margin: '0 auto' }}>
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
          MCP security is a ticking time bomb
        </h2>
      </div>

      <div style={{ display: 'flex', flexDirection: isMobile ? 'column' : 'row', gap: isMobile ? 16 : 20 }}>
        <StatCard target={30} suffix="+" label="CVEs filed in 60 days" delay={0} isMobile={isMobile} />
        <StatCard target={437} suffix="K" label="Compromised downloads" delay={0.1} isMobile={isMobile} />
        <StatCard target={400} suffix="M+" label="Raised by competitors in 90 days" delay={0.2} isMobile={isMobile} />
      </div>
    </section>
  )
}
