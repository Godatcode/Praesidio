import { Navbar } from '../landing/Navbar'
import { Hero } from '../landing/Hero'
import { Showcase } from '../landing/Showcase'
import { Problem } from '../landing/Problem'
import { Features } from '../landing/Features'
import { HowItWorks } from '../landing/HowItWorks'
import { Comparison } from '../landing/Comparison'
import { CTA } from '../landing/CTA'
import { Footer } from '../landing/Footer'
import { useAnchorScroll } from '../hooks/useAnchorScroll'

export default function LandingPage() {
  useAnchorScroll()

  return (
    <div className="landing-page" style={{ background: '#08080c', color: '#f0f0f2', minHeight: '100vh' }}>
      <Navbar />
      <Hero />
      <Showcase />
      <div className="section-divider" />
      <Problem />
      <div className="section-divider" />
      <Features />
      <div className="section-divider" />
      <HowItWorks />
      <div className="section-divider" />
      <Comparison />
      <div className="section-divider" />
      <CTA />
      <Footer />
    </div>
  )
}
