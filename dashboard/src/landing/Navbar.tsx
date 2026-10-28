import { useState, useEffect } from 'react'
import { useMediaQuery, MOBILE_QUERY } from '../hooks/useMediaQuery'

export function Navbar() {
  const [scrolled, setScrolled] = useState(false)
  const isMobile = useMediaQuery(MOBILE_QUERY)

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 20)
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  }, [])

  return (
    <nav
      style={{
        position: 'fixed',
        top: 0,
        left: 0,
        right: 0,
        zIndex: 100,
        padding: isMobile ? '0 16px' : '0 40px',
        height: 64,
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
        transition: 'all 0.3s ease',
        background: scrolled ? 'rgba(8, 8, 12, 0.8)' : 'transparent',
        backdropFilter: scrolled ? 'blur(16px)' : 'none',
        borderBottom: scrolled ? '1px solid rgba(255,255,255,0.06)' : '1px solid transparent',
      }}
    >
      <div style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
        <img
          src="/logo.png"
          alt="Praesidio"
          style={{
            width: 32,
            height: 32,
            borderRadius: 7,
            objectFit: 'cover',
          }}
        />
        <span style={{
          fontFamily: "'Clash Display', sans-serif",
          fontWeight: 600,
          fontSize: 16,
          color: '#f0f0f2',
          letterSpacing: '-0.02em',
        }}>
          Praesidio
        </span>
      </div>

      <div style={{ display: 'flex', alignItems: 'center', gap: 32 }}>
        {/* Section links don't fit beside the logo on phones; the CTA alone stays visible */}
        {!isMobile && (
          <>
            <a
              href="#features"
              className="navbar-link"
              style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none', transition: 'color 0.2s' }}
            >
              Features
            </a>
            <a
              href="#how-it-works"
              className="navbar-link"
              style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none', transition: 'color 0.2s' }}
            >
              How It Works
            </a>
            <a
              href="#comparison"
              className="navbar-link"
              style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none', transition: 'color 0.2s' }}
            >
              Compare
            </a>
          </>
        )}
        <a
          href="#cta"
          style={{
            fontSize: 13,
            fontWeight: 500,
            color: '#f0f0f2',
            textDecoration: 'none',
            whiteSpace: 'nowrap',
            padding: '7px 16px',
            borderRadius: 8,
            background: 'linear-gradient(135deg, #7c5bf0 0%, #4a8bf5 100%)',
            border: '1px solid rgba(124, 91, 240, 0.3)',
            transition: 'all 0.2s ease',
            display: 'inline-flex',
            alignItems: 'center',
            gap: 6,
          }}
        >
          Get Early Access
        </a>
      </div>
    </nav>
  )
}
