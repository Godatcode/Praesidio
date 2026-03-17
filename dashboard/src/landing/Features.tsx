import { useLandingReveal, useStaggerReveal } from '../hooks/useLandingReveal'
import { Shield, Brain, Activity, Globe, Bug, CheckCircle } from 'lucide-react'

const features = [
  {
    icon: Shield,
    color: '#7c5bf0',
    title: 'Runtime Firewall',
    desc: 'Intercepts every MCP tool call in real time. Scans inputs and outputs for injection, exfiltration, and credential leaks before they reach your LLM.',
  },
  {
    icon: Brain,
    color: '#4a8bf5',
    title: 'LLM Analysis',
    desc: 'Claude-powered semantic analysis of tool descriptions. Detects adversarial patterns that regex alone would miss — hidden instructions, prompt overrides, social engineering.',
  },
  {
    icon: Activity,
    color: '#22c55e',
    title: 'Behavioral Detection',
    desc: 'Establishes baselines for every tool — response times, output sizes, call frequency. Flags anomalies before they become breaches.',
  },
  {
    icon: Globe,
    color: '#f59e0b',
    title: 'Threat Intelligence',
    desc: '12 built-in threat signatures covering OWASP MCP Top 10 and Agentic Top 10. Pattern matching, unicode detection, supply chain verification.',
  },
  {
    icon: Bug,
    color: '#ef4444',
    title: 'Honeypot Server',
    desc: 'Deploy decoy tools that look real to attackers. Log every interaction. Understand attacker behavior without any risk to production.',
  },
  {
    icon: CheckCircle,
    color: '#8b5cf6',
    title: 'OWASP Coverage',
    desc: 'Maps every security module to OWASP MCP Top 10 and Agentic Top 10 frameworks. Generate compliance reports with one click.',
  },
]

export function Features() {
  const titleRef = useLandingReveal({ y: 20 })
  const gridRef = useStaggerReveal('.feature-card', 100)

  return (
    <section id="features" style={{ padding: '120px 40px', maxWidth: 1100, margin: '0 auto' }}>
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
          Everything you need to secure AI agents
        </h2>
      </div>

      <div
        ref={gridRef}
        style={{
          display: 'grid',
          gridTemplateColumns: 'repeat(3, 1fr)',
          gap: 20,
        }}
      >
        {features.map((feature) => {
          const Icon = feature.icon
          return (
            <div
              key={feature.title}
              className="feature-card"
              style={{
                padding: 32,
                background: 'rgba(255,255,255,0.05)',
                border: '1px solid rgba(255,255,255,0.08)',
                borderRadius: 16,
                cursor: 'default',
              }}
            >
              <div
                style={{
                  width: 40,
                  height: 40,
                  borderRadius: 10,
                  background: `${feature.color}15`,
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  marginBottom: 20,
                }}
              >
                <Icon size={18} color={feature.color} />
              </div>
              <h3 style={{
                fontFamily: "'Clash Display', sans-serif",
                fontSize: 18,
                fontWeight: 500,
                color: '#f0f0f2',
                marginBottom: 8,
                letterSpacing: '-0.02em',
              }}>
                {feature.title}
              </h3>
              <p style={{ fontSize: 13, lineHeight: 1.6, color: 'rgba(255, 255, 255, 0.6)' }}>{feature.desc}</p>
            </div>
          )
        })}
      </div>
    </section>
  )
}
