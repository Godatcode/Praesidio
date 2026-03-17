import { useState } from 'react'
import { useLandingReveal } from '../hooks/useLandingReveal'
import { ArrowRight } from 'lucide-react'

export function CTA() {
  const [email, setEmail] = useState('')
  const [submitted, setSubmitted] = useState(false)
  const sectionRef = useLandingReveal({ y: 30 })

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault()
    if (email.includes('@')) setSubmitted(true)
  }

  return (
    <section id="cta" style={{ padding: '120px 40px 80px' }}>
      <div
        ref={sectionRef}
        style={{
          maxWidth: 700,
          margin: '0 auto',
          textAlign: 'center',
          padding: '80px 48px',
          background: 'rgba(255,255,255,0.05)',
          border: '1px solid rgba(255,255,255,0.08)',
          borderRadius: 20,
          position: 'relative',
          overflow: 'hidden',
        }}
      >
        {/* Subtle orb */}
        <div
          style={{
            position: 'absolute',
            width: 400,
            height: 400,
            borderRadius: '50%',
            background: 'radial-gradient(circle, rgba(124, 91, 240, 0.1) 0%, transparent 70%)',
            filter: 'blur(60px)',
            top: '50%',
            left: '50%',
            transform: 'translate(-50%, -50%)',
            pointerEvents: 'none',
          }}
        />

        <div style={{ position: 'relative', zIndex: 1 }}>
          <h2 style={{
            fontFamily: "'Clash Display', sans-serif",
            fontSize: 36,
            fontWeight: 500,
            letterSpacing: '-0.03em',
            color: '#f0f0f2',
            lineHeight: 1.15,
            marginBottom: 16,
          }}>
            Ready to protect your AI agents?
          </h2>
          <p style={{ fontSize: 15, color: 'rgba(255,255,255,0.5)', marginBottom: 40 }}>
            Free for developers. Team plans from $49/mo.
          </p>

          {submitted ? (
            <div style={{
              padding: '16px 32px',
              borderRadius: 10,
              background: 'rgba(34, 197, 94, 0.1)',
              border: '1px solid rgba(34, 197, 94, 0.2)',
              color: '#22c55e',
              fontSize: 14,
              fontWeight: 500,
            }}>
              You&apos;re on the list. We&apos;ll be in touch.
            </div>
          ) : (
            <form onSubmit={handleSubmit} style={{ display: 'flex', gap: 12, justifyContent: 'center' }}>
              <input
                type="email"
                placeholder="you@company.com"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                style={{
                  width: 300,
                  padding: '12px 20px',
                  borderRadius: 10,
                  border: '1px solid rgba(255,255,255,0.1)',
                  background: 'rgba(255,255,255,0.04)',
                  color: '#f0f0f2',
                  fontSize: 14,
                  fontFamily: "'Inter', sans-serif",
                  outline: 'none',
                  transition: 'border-color 0.2s',
                }}
                onFocus={(e) => e.currentTarget.style.borderColor = 'rgba(124, 91, 240, 0.4)'}
                onBlur={(e) => e.currentTarget.style.borderColor = 'rgba(255,255,255,0.1)'}
              />
              <button
                type="submit"
                className="landing-cta-primary"
                style={{
                  display: 'inline-flex',
                  alignItems: 'center',
                  gap: 8,
                  padding: '12px 24px',
                  borderRadius: 10,
                  background: 'linear-gradient(135deg, #7c5bf0 0%, #4a8bf5 100%)',
                  color: '#fff',
                  fontSize: 14,
                  fontWeight: 500,
                  border: 'none',
                  cursor: 'pointer',
                }}
              >
                Get Early Access <ArrowRight size={14} />
              </button>
            </form>
          )}
        </div>
      </div>
    </section>
  )
}
