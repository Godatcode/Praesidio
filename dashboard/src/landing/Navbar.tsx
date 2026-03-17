import { useState, useEffect } from 'react'
import { Link } from 'react-router-dom'
import { ShieldCheck } from 'lucide-react'

export function Navbar() {
  const [scrolled, setScrolled] = useState(false)

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
        padding: '0 40px',
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
        <div style={{
          width: 28,
          height: 28,
          background: '#7c5bf0',
          borderRadius: 7,
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}>
          <ShieldCheck size={14} color="#fff" />
        </div>
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
        <Link
          to="/app"
          style={{
            fontSize: 13,
            fontWeight: 500,
            color: '#f0f0f2',
            textDecoration: 'none',
            padding: '7px 16px',
            borderRadius: 8,
            background: 'rgba(124, 91, 240, 0.12)',
            border: '1px solid rgba(124, 91, 240, 0.2)',
            transition: 'all 0.2s ease',
          }}
        >
          Dashboard
        </Link>
      </div>
    </nav>
  )
}
