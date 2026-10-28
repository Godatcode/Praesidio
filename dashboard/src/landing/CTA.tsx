import { useState } from 'react'
import { useLandingReveal } from '../hooks/useLandingReveal'
import { ArrowRight, Loader2 } from 'lucide-react'
import { useMediaQuery, MOBILE_QUERY } from '../hooks/useMediaQuery'

const API_BASE = import.meta.env.VITE_API_URL || ''

export function CTA() {
  const [email, setEmail] = useState('')
  const [status, setStatus] = useState<'idle' | 'loading' | 'success' | 'already' | 'error'>('idle')
  const [errorMsg, setErrorMsg] = useState('')
  const sectionRef = useLandingReveal({ y: 30 })
  const isMobile = useMediaQuery(MOBILE_QUERY)

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    if (!email.includes('@') || !email.includes('.')) {
      setErrorMsg('Please enter a valid email address.')
      setStatus('error')
      return
    }

    setStatus('loading')
    setErrorMsg('')

    try {
      const resp = await fetch(`${API_BASE}/api/waitlist`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: email.trim(), source: 'landing_cta' }),
      })

      const data = await resp.json()

      if (resp.ok) {
        setStatus(data.new ? 'success' : 'already')
      } else {
        setErrorMsg(data.error || 'Something went wrong.')
        setStatus('error')
      }
    } catch {
      setErrorMsg('Could not connect. Please try again.')
      setStatus('error')
    }
  }

  return (
    <section id="cta" style={{ padding: isMobile ? '80px 16px 60px' : '120px 40px 80px' }}>
      <div
        ref={sectionRef}
        className="cta-glass"
        style={{
          maxWidth: 700,
          margin: '0 auto',
          textAlign: 'center',
          padding: isMobile ? '48px 20px' : '80px 48px',
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
            fontSize: isMobile ? 28 : 36,
            fontWeight: 500,
            letterSpacing: '-0.03em',
            color: '#f0f0f2',
            lineHeight: 1.15,
            marginBottom: 16,
          }}>
            Ready to protect your AI agents?
          </h2>
          <p style={{ fontSize: 15, color: 'rgba(255,255,255,0.5)', marginBottom: 40 }}>
            Be the first to know when we launch.
          </p>

          {status === 'success' || status === 'already' ? (
            <div style={{
              padding: '16px 32px',
              borderRadius: 10,
              background: 'rgba(34, 197, 94, 0.1)',
              border: '1px solid rgba(34, 197, 94, 0.2)',
              color: '#22c55e',
              fontSize: 14,
              fontWeight: 500,
            }}>
              {status === 'success'
                ? "You're on the list! Check your inbox for a welcome email."
                : "You're already on the list. We'll be in touch!"}
            </div>
          ) : (
            <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12 }}>
              <div style={{
                display: 'flex',
                flexDirection: isMobile ? 'column' : 'row',
                gap: 12,
                justifyContent: 'center',
                width: isMobile ? '100%' : undefined,
              }}>
                <input
                  type="email"
                  required
                  placeholder="you@company.com"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  disabled={status === 'loading'}
                  style={{
                    width: isMobile ? '100%' : 300,
                    boxSizing: 'border-box',
                    padding: '12px 20px',
                    borderRadius: 10,
                    border: '1px solid rgba(255,255,255,0.1)',
                    background: 'rgba(255,255,255,0.04)',
                    color: '#f0f0f2',
                    fontSize: 14,
                    fontFamily: "'Inter', sans-serif",
                    outline: 'none',
                    transition: 'border-color 0.2s',
                    opacity: status === 'loading' ? 0.6 : 1,
                  }}
                  onFocus={(e) => e.currentTarget.style.borderColor = 'rgba(124, 91, 240, 0.4)'}
                  onBlur={(e) => e.currentTarget.style.borderColor = 'rgba(255,255,255,0.1)'}
                />
                <button
                  type="submit"
                  disabled={status === 'loading'}
                  className="landing-cta-primary"
                  style={{
                    display: 'inline-flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    gap: 8,
                    padding: '12px 24px',
                    borderRadius: 10,
                    background: 'linear-gradient(135deg, #7c5bf0 0%, #4a8bf5 100%)',
                    color: '#fff',
                    fontSize: 14,
                    fontWeight: 500,
                    border: 'none',
                    cursor: status === 'loading' ? 'wait' : 'pointer',
                    opacity: status === 'loading' ? 0.7 : 1,
                  }}
                >
                  {status === 'loading' ? (
                    <>Joining... <Loader2 size={14} className="animate-spin" /></>
                  ) : (
                    <>Get Early Access <ArrowRight size={14} /></>
                  )}
                </button>
              </div>
              {status === 'error' && (
                <p style={{ color: '#ef4444', fontSize: 13, marginTop: 4 }}>
                  {errorMsg}
                </p>
              )}
            </form>
          )}
        </div>
      </div>
    </section>
  )
}
