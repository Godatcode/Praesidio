import { useCallback, useRef } from 'react'
import { useLandingReveal } from '../hooks/useLandingReveal'
import { ArrowRight } from 'lucide-react'
import { Link } from 'react-router-dom'
import { ParticleCanvas } from './ParticleCanvas'

const logos = ['Anthropic', 'Cursor', 'OpenAI', 'Vercel', 'Stripe', 'GitHub', 'Cloudflare', 'Linear']

function LogoMarquee() {
  return (
    <div className="marquee-container" style={{ overflow: 'hidden', width: '100%', marginTop: 80 }}>
      <div
        style={{
          display: 'flex',
          gap: 64,
          animation: 'marquee 30s linear infinite',
          width: 'max-content',
        }}
      >
        {[...logos, ...logos, ...logos].map((logo, i) => (
          <span
            key={i}
            style={{
              fontSize: 14,
              fontWeight: 500,
              color: 'rgba(255,255,255,0.15)',
              whiteSpace: 'nowrap',
              letterSpacing: '0.02em',
            }}
          >
            {logo}
          </span>
        ))}
      </div>
    </div>
  )
}

export function Hero() {
  const previewRef = useLandingReveal({ y: 50, delay: 0.3, scale: 0.97 })
  const tiltRef = useRef<HTMLDivElement>(null)

  const handleMouseMove = useCallback((e: React.MouseEvent<HTMLDivElement>) => {
    const el = tiltRef.current
    if (!el) return
    const rect = el.getBoundingClientRect()
    const x = (e.clientX - rect.left) / rect.width - 0.5
    const y = (e.clientY - rect.top) / rect.height - 0.5
    el.style.transform = `rotateY(${x * 6}deg) rotateX(${-y * 6}deg)`
  }, [])

  const handleMouseLeave = useCallback(() => {
    const el = tiltRef.current
    if (!el) return
    el.style.transition = 'transform 0.5s ease'
    el.style.transform = 'rotateX(4deg) rotateY(0deg)'
    setTimeout(() => { if (el) el.style.transition = 'transform 0.1s ease' }, 500)
  }, [])

  return (
    <section
      style={{
        position: 'relative',
        minHeight: '100vh',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        textAlign: 'center',
        padding: '140px 40px 80px',
        overflow: 'hidden',
      }}
    >
      {/* Particle canvas background */}
      <ParticleCanvas />

      {/* Breathing gradient orb */}
      <div
        className="hero-orb"
        style={{
          position: 'absolute',
          width: 800,
          height: 800,
          borderRadius: '50%',
          background: 'radial-gradient(circle, rgba(124, 91, 240, 0.15) 0%, rgba(74, 139, 245, 0.08) 40%, transparent 70%)',
          filter: 'blur(80px)',
          top: -200,
          left: '50%',
          pointerEvents: 'none',
          zIndex: 0,
        }}
      />

      {/* Grid lines */}
      <div
        style={{
          position: 'absolute',
          inset: 0,
          backgroundImage:
            'linear-gradient(rgba(255,255,255,0.03) 1px, transparent 1px), linear-gradient(90deg, rgba(255,255,255,0.03) 1px, transparent 1px)',
          backgroundSize: '80px 80px',
          maskImage: 'radial-gradient(ellipse 50% 50% at 50% 50%, black, transparent)',
          WebkitMaskImage: 'radial-gradient(ellipse 50% 50% at 50% 50%, black, transparent)',
          pointerEvents: 'none',
          zIndex: 0,
        }}
      />

      <div className="hero-stagger" style={{ position: 'relative', zIndex: 1, maxWidth: 900 }}>
        {/* Pill badge */}
        <div>
          <span
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 8,
              padding: '6px 16px',
              borderRadius: 100,
              border: '1px solid rgba(124, 91, 240, 0.25)',
              background: 'rgba(124, 91, 240, 0.08)',
              fontSize: 12,
              fontWeight: 500,
              color: '#a78bfa',
              marginBottom: 32,
              letterSpacing: '0.02em',
            }}
          >
            <span>🛡️</span> Runtime AI Agent Security
          </span>
        </div>

        <h1
          style={{
            fontFamily: "'Clash Display', sans-serif",
            fontSize: 72,
            fontWeight: 600,
            lineHeight: 1.05,
            letterSpacing: '-0.03em',
            color: '#f0f0f2',
            maxWidth: 800,
            margin: '0 auto',
          }}
        >
          Stop trusting your{' '}
          <span
            style={{
              background: 'linear-gradient(135deg, #a78bfa, #60a5fa, #34d399)',
              WebkitBackgroundClip: 'text',
              WebkitTextFillColor: 'transparent',
              backgroundClip: 'text',
            }}
          >
            AI agents
          </span>{' '}
          blindly.
        </h1>

        <p
          style={{
            fontSize: 18,
            lineHeight: 1.6,
            color: 'rgba(255,255,255,0.5)',
            maxWidth: 580,
            margin: '24px auto 0',
          }}
        >
          30+ MCP CVEs in 60 days. 437K compromised downloads. Praesidio is the runtime firewall your AI agents need.
        </p>

        {/* CTA buttons */}
        <div
          style={{
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            gap: 16,
            marginTop: 40,
          }}
        >
          <a
            href="#cta"
            className="landing-cta-primary"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 8,
              padding: '12px 28px',
              borderRadius: 10,
              background: 'linear-gradient(135deg, #7c5bf0 0%, #4a8bf5 100%)',
              color: '#fff',
              fontSize: 14,
              fontWeight: 500,
              textDecoration: 'none',
              transition: 'all 0.2s ease',
              border: 'none',
              cursor: 'pointer',
            }}
          >
            Get Early Access <ArrowRight size={14} />
          </a>
          <Link
            to="/app"
            className="landing-cta-secondary"
            style={{
              display: 'inline-flex',
              alignItems: 'center',
              gap: 8,
              padding: '12px 28px',
              borderRadius: 10,
              background: 'transparent',
              color: 'rgba(255,255,255,0.6)',
              fontSize: 14,
              fontWeight: 500,
              textDecoration: 'none',
              transition: 'all 0.2s ease',
              border: '1px solid rgba(255,255,255,0.1)',
              cursor: 'pointer',
            }}
          >
            View Dashboard
          </Link>
        </div>

        {/* Dashboard preview with 3D mouse-tilt */}
        <div ref={previewRef} style={{ marginTop: 80, perspective: 1200 }}>
          <div
            ref={tiltRef}
            onMouseMove={handleMouseMove}
            onMouseLeave={handleMouseLeave}
            style={{
              transform: 'rotateX(4deg)',
              borderRadius: 16,
              border: '1px solid rgba(255,255,255,0.1)',
              boxShadow: '0 40px 100px -20px rgba(124, 91, 240, 0.15), 0 20px 60px -10px rgba(0,0,0,0.5)',
              overflow: 'hidden',
              background: '#0e0e14',
              padding: 2,
              transition: 'transform 0.1s ease',
            }}
          >
            <div
              style={{
                borderRadius: 14,
                overflow: 'hidden',
                background: '#111113',
                position: 'relative',
              }}
            >
              {/* macOS chrome */}
              <div style={{ padding: '16px 20px', borderBottom: '1px solid rgba(255,255,255,0.06)', display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={{ width: 10, height: 10, borderRadius: '50%', background: '#ef4444', opacity: 0.8 }} />
                <span style={{ width: 10, height: 10, borderRadius: '50%', background: '#f59e0b', opacity: 0.8 }} />
                <span style={{ width: 10, height: 10, borderRadius: '50%', background: '#22c55e', opacity: 0.8 }} />
                <span style={{ flex: 1, textAlign: 'center', fontSize: 11, color: 'rgba(255,255,255,0.2)', fontFamily: "'JetBrains Mono', monospace" }}>praesidio — dashboard</span>
              </div>
              {/* Metric cards */}
              <div style={{ padding: 32, display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: 16 }}>
                {[
                  { label: 'SERVERS', value: '7' },
                  { label: 'TOOLS PINNED', value: '21' },
                  { label: 'THREATS BLOCKED', value: '3' },
                  { label: 'OWASP SCORE', value: '9/10' },
                ].map((card) => (
                  <div key={card.label} style={{ background: 'rgba(255,255,255,0.05)', border: '1px solid rgba(255,255,255,0.08)', borderRadius: 12, padding: 20 }}>
                    <div style={{ fontSize: 10, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', marginBottom: 10 }}>{card.label}</div>
                    <div style={{ fontFamily: "'JetBrains Mono', monospace", fontSize: 28, fontWeight: 500, color: '#f0f0f2' }}>{card.value}</div>
                  </div>
                ))}
              </div>
              {/* Feed + chart */}
              <div style={{ padding: '0 32px 32px', display: 'grid', gridTemplateColumns: '3fr 2fr', gap: 16 }}>
                <div style={{ background: 'rgba(255,255,255,0.05)', border: '1px solid rgba(255,255,255,0.08)', borderRadius: 12, padding: 20, height: 160 }}>
                  <div style={{ fontSize: 10, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', marginBottom: 12 }}>EVENT FEED</div>
                  {[
                    { color: '#ef4444', text: 'Tool poisoning detected — sketchy-math.calculate' },
                    { color: '#ef4444', text: 'Credential leak blocked — github.read_file' },
                    { color: '#f59e0b', text: 'Sensitive path access — filesystem.read_file' },
                    { color: '#22c55e', text: 'Scan completed — clean' },
                  ].map((row, i) => (
                    <div key={i} style={{ display: 'flex', alignItems: 'center', gap: 10, padding: '6px 0' }}>
                      <span style={{ width: 5, height: 5, borderRadius: '50%', background: row.color, flexShrink: 0 }} />
                      <span style={{ fontSize: 11, color: 'rgba(255,255,255,0.3)', fontFamily: "'JetBrains Mono', monospace", overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{row.text}</span>
                    </div>
                  ))}
                </div>
                <div style={{ background: 'rgba(255,255,255,0.05)', border: '1px solid rgba(255,255,255,0.08)', borderRadius: 12, padding: 20, height: 160 }}>
                  <div style={{ fontSize: 10, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', marginBottom: 12 }}>THREAT TREND</div>
                  <svg width="100%" height="100" viewBox="0 0 300 100" preserveAspectRatio="none">
                    <path d="M0,80 Q30,70 60,65 T120,50 T180,55 T240,30 T300,40" fill="none" stroke="#22c55e" strokeWidth="1.5" opacity="0.6" />
                    <path d="M0,90 Q30,85 60,88 T120,80 T180,85 T240,75 T300,82" fill="none" stroke="#f59e0b" strokeWidth="1.5" opacity="0.6" />
                    <path d="M0,95 Q30,93 60,95 T120,92 T180,95 T240,88 T300,95" fill="none" stroke="#ef4444" strokeWidth="1.5" opacity="0.6" />
                  </svg>
                </div>
              </div>
            </div>
          </div>
        </div>

        <LogoMarquee />
      </div>
    </section>
  )
}
