import { useEffect, useRef, useState } from 'react'
import { useLandingReveal } from '../hooks/useLandingReveal'

function TypewriterLine({ text, delay, color, prompt }: { text: string; delay: number; color?: string; prompt?: boolean }) {
  const [displayed, setDisplayed] = useState('')
  const [done, setDone] = useState(false)
  const ref = useRef<HTMLDivElement>(null)
  const started = useRef(false)

  useEffect(() => {
    const el = ref.current
    if (!el) return
    const obs = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting && !started.current) {
          started.current = true
          obs.disconnect()
          if (!text) { setDone(true); return }
          let i = 0
          const timer = setTimeout(() => {
            const interval = setInterval(() => {
              i++
              setDisplayed(text.slice(0, i))
              if (i >= text.length) {
                clearInterval(interval)
                setDone(true)
              }
            }, prompt ? 40 : 15)
          }, delay)
          return () => clearTimeout(timer)
        }
      },
      { threshold: 0.3 },
    )
    obs.observe(el)
    return () => obs.disconnect()
  }, [text, delay, prompt])

  return (
    <div ref={ref} style={{ color: color ?? 'rgba(255,255,255,0.4)', minHeight: text ? undefined : 8 }}>
      {prompt && <span style={{ color: '#7c5bf0' }}>$ </span>}
      {displayed}
      {prompt && !done && <span className="typing-cursor" />}
    </div>
  )
}

const terminals = [
  {
    title: 'Install',
    lines: [
      { prompt: true, text: 'cargo install praesidio' },
      { prompt: false, text: '    Compiling praesidio v0.2.0' },
      { prompt: false, text: '    Finished release [optimized] target(s)' },
      { prompt: false, text: '  Installing ~/.cargo/bin/praesidio', color: '#22c55e' },
      { prompt: false, text: '   Installed package `praesidio v0.2.0`', color: '#22c55e' },
    ],
  },
  {
    title: 'Scan',
    lines: [
      { prompt: true, text: 'praesidio scan' },
      { prompt: false, text: '' },
      { prompt: false, text: 'Scanning 7 servers, 23 tools...', color: 'rgba(255,255,255,0.4)' },
      { prompt: false, text: '  \u2713 github         \u2014 5 tools, trust 96/100', color: '#22c55e' },
      { prompt: false, text: '  \u2713 filesystem     \u2014 4 tools, trust 94/100', color: '#22c55e' },
      { prompt: false, text: '  \u2713 postgres       \u2014 3 tools, trust 91/100', color: '#22c55e' },
      { prompt: false, text: '  \u26a0 custom-api     \u2014 4 tools, trust 73/100', color: '#f59e0b' },
      { prompt: false, text: '  \u2717 sketchy-math   \u2014 BLOCKED \u2014 tool poisoning detected', color: '#ef4444' },
      { prompt: false, text: '' },
      { prompt: false, text: '3 threats blocked. 21/23 tools pinned.', color: '#f0f0f2' },
    ],
  },
  {
    title: 'Protect',
    lines: [
      { prompt: true, text: 'praesidio proxy --port 3100' },
      { prompt: false, text: '' },
      { prompt: false, text: '\u2713 Proxy active on :3100', color: '#22c55e' },
      { prompt: false, text: '\u2713 All MCP traffic secured', color: '#22c55e' },
      { prompt: false, text: '\u2713 Output filtering enabled', color: '#22c55e' },
      { prompt: false, text: '\u2713 Behavioral baselines loaded', color: '#22c55e' },
      { prompt: false, text: '' },
      { prompt: false, text: 'Watching for threats...', color: 'rgba(255,255,255,0.4)' },
    ],
  },
]

function Terminal({ terminal, index }: { terminal: typeof terminals[0]; index: number }) {
  const ref = useLandingReveal({ y: 30, delay: index * 0.15 })

  return (
    <div ref={ref} style={{ flex: 1 }}>
      <div
        style={{
          background: '#0e0e14',
          border: '1px solid rgba(255,255,255,0.08)',
          borderRadius: 12,
          overflow: 'hidden',
        }}
      >
        {/* macOS chrome */}
        <div style={{
          display: 'flex',
          alignItems: 'center',
          gap: 6,
          padding: '12px 16px',
          borderBottom: '1px solid rgba(255,255,255,0.06)',
        }}>
          <span style={{ width: 10, height: 10, borderRadius: '50%', background: '#ef4444', opacity: 0.7 }} />
          <span style={{ width: 10, height: 10, borderRadius: '50%', background: '#f59e0b', opacity: 0.7 }} />
          <span style={{ width: 10, height: 10, borderRadius: '50%', background: '#22c55e', opacity: 0.7 }} />
          <span style={{ flex: 1, textAlign: 'center', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontFamily: "'JetBrains Mono', monospace" }}>
            {terminal.title}
          </span>
        </div>
        {/* Terminal content with typing animation */}
        <div style={{ padding: '16px 20px', fontFamily: "'JetBrains Mono', monospace", fontSize: 12, lineHeight: 1.7 }}>
          {terminal.lines.map((line, i) => (
            <TypewriterLine
              key={i}
              text={line.text}
              delay={i * 120}
              color={line.color}
              prompt={line.prompt}
            />
          ))}
        </div>
      </div>
    </div>
  )
}

function ConnectingPath() {
  const pathRef = useRef<SVGPathElement>(null)

  useEffect(() => {
    const path = pathRef.current
    if (!path) return

    const length = path.getTotalLength()
    path.style.strokeDasharray = `${length}`
    path.style.strokeDashoffset = `${length}`
    path.style.transition = 'stroke-dashoffset 1.5s cubic-bezier(0.16, 1, 0.3, 1)'

    const obs = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          path.style.strokeDashoffset = '0'
          obs.disconnect()
        }
      },
      { threshold: 0.3 },
    )
    obs.observe(path)
    return () => obs.disconnect()
  }, [])

  return (
    <svg
      style={{ position: 'absolute', top: 20, left: '15%', width: '70%', height: 2, overflow: 'visible' }}
      viewBox="0 0 700 2"
      preserveAspectRatio="none"
    >
      <path
        ref={pathRef}
        d="M0,1 L700,1"
        fill="none"
        stroke="rgba(124, 91, 240, 0.3)"
        strokeWidth="1"
        strokeDasharray="6 6"
      />
    </svg>
  )
}

export function HowItWorks() {
  const titleRef = useLandingReveal({ y: 20 })

  return (
    <section id="how-it-works" style={{ padding: '120px 40px', maxWidth: 1100, margin: '0 auto' }}>
      <div ref={titleRef} style={{ textAlign: 'center', marginBottom: 64 }}>
        <span style={{
          fontSize: 11,
          fontWeight: 500,
          textTransform: 'uppercase',
          letterSpacing: '0.08em',
          color: 'rgba(255,255,255,0.4)',
          display: 'block',
          marginBottom: 16,
        }}>
          How it works
        </span>
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
          Three commands. Full protection.
        </h2>
      </div>

      {/* Step numbers with connecting path */}
      <div style={{ position: 'relative' }}>
        <div style={{ display: 'flex', justifyContent: 'space-around', marginBottom: 32 }}>
          {['01', '02', '03'].map((num) => (
            <div
              key={num}
              style={{
                width: 40,
                height: 40,
                borderRadius: '50%',
                border: '1px solid rgba(124, 91, 240, 0.3)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontFamily: "'JetBrains Mono', monospace",
                fontSize: 12,
                color: '#7c5bf0',
                background: 'rgba(124, 91, 240, 0.08)',
                zIndex: 1,
                position: 'relative',
              }}
            >
              {num}
            </div>
          ))}
        </div>
        <ConnectingPath />
      </div>

      <div style={{ display: 'flex', gap: 20 }}>
        {terminals.map((t, i) => (
          <Terminal key={t.title} terminal={t} index={i} />
        ))}
      </div>
    </section>
  )
}
