import { Link } from 'react-router-dom'

const REPO_URL = 'https://github.com/Godatcode/Praesidio'

const linkStyle = { fontSize: 13, color: '#8a8a9a', textDecoration: 'none' }

export function Footer() {
  return (
    <footer
      style={{
        padding: '60px 40px 40px',
        borderTop: '1px solid rgba(255,255,255,0.06)',
        maxWidth: 1100,
        margin: '0 auto',
      }}
    >
      <div style={{ display: 'flex', flexWrap: 'wrap', gap: 40, alignItems: 'flex-start', justifyContent: 'space-between', marginBottom: 40 }}>
        <div>
          <div style={{ display: 'flex', alignItems: 'center', gap: 10, marginBottom: 12 }}>
            <img
              src="/logo.png"
              alt="Praesidio"
              style={{
                width: 28,
                height: 28,
                borderRadius: 6,
                objectFit: 'cover',
              }}
            />
            <span style={{
              fontFamily: "'Clash Display', sans-serif",
              fontWeight: 600,
              fontSize: 15,
              color: '#f0f0f2',
            }}>
              Praesidio
            </span>
          </div>
          <p style={{ fontSize: 13, color: '#8a8a9a', maxWidth: 300, lineHeight: 1.6 }}>
            Built with Rust. Secured by design. Runtime protection for the agentic era.
          </p>
        </div>

        <div style={{ display: 'flex', flexWrap: 'wrap', gap: 48 }}>
          <div>
            <div style={{ fontSize: 11, fontWeight: 500, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', marginBottom: 16 }}>Product</div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
              <a href="#features" style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none' }}>Features</a>
              <a href="#comparison" style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none' }}>Compare</a>
              <Link to="/app" style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none' }}>Dashboard</Link>
            </div>
          </div>
          <div>
            <div style={{ fontSize: 11, fontWeight: 500, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', marginBottom: 16 }}>Resources</div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
              <a href="#how-it-works" style={{ fontSize: 13, color: '#8a8a9a', textDecoration: 'none' }}>Documentation</a>
              <a href={`${REPO_URL}/tree/main/rules`} target="_blank" rel="noopener noreferrer" style={linkStyle}>Threat Database</a>
              <a href={`${REPO_URL}#owasp-coverage`} target="_blank" rel="noopener noreferrer" style={linkStyle}>OWASP Mapping</a>
            </div>
          </div>
          <div>
            <div style={{ fontSize: 11, fontWeight: 500, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(255,255,255,0.3)', marginBottom: 16 }}>Company</div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>
              <a href={REPO_URL} target="_blank" rel="noopener noreferrer" style={linkStyle}>GitHub</a>
              <a href={`${REPO_URL}/issues`} target="_blank" rel="noopener noreferrer" style={linkStyle}>Contact</a>
            </div>
          </div>
        </div>
      </div>

      <div style={{
        paddingTop: 24,
        borderTop: '1px solid rgba(255,255,255,0.04)',
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'space-between',
      }}>
        <span style={{ fontSize: 12, color: 'rgba(255,255,255,0.2)' }}>
          &copy; 2026 Praesidio. All rights reserved.
        </span>
        <span style={{ fontSize: 12, color: 'rgba(255,255,255,0.15)', fontFamily: "'JetBrains Mono', monospace" }}>
          v0.2.0
        </span>
      </div>
    </footer>
  )
}
