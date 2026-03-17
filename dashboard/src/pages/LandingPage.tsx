import { Navbar } from '../landing/Navbar'
import { Hero } from '../landing/Hero'
import { Problem } from '../landing/Problem'
import { Features } from '../landing/Features'
import { HowItWorks } from '../landing/HowItWorks'
import { Comparison } from '../landing/Comparison'
import { CTA } from '../landing/CTA'
import { Footer } from '../landing/Footer'

export default function LandingPage() {
  return (
    <div className="landing-page" style={{ background: '#08080c', color: '#f0f0f2', minHeight: '100vh' }}>
      <Navbar />
      <Hero />
      <Problem />
      <Features />
      <HowItWorks />
      <Comparison />
      <CTA />
      <Footer />
    </div>
  )
}
