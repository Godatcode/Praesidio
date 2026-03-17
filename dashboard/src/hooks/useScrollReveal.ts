import { useEffect, useRef } from 'react'
import gsap from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'

gsap.registerPlugin(ScrollTrigger)

interface RevealOptions {
  y?: number
  x?: number
  scale?: number
  rotation?: number
  delay?: number
  duration?: number
}

export function useScrollReveal(options: RevealOptions = {}) {
  const ref = useRef<HTMLDivElement>(null)
  const { y = 40, x = 0, scale = 1, rotation = 0, delay = 0, duration = 0.8 } = options

  useEffect(() => {
    const el = ref.current
    if (!el) return

    gsap.set(el, { opacity: 0, y, x, scale, rotation })

    const st = gsap.to(el, {
      opacity: 1,
      y: 0,
      x: 0,
      scale: 1,
      rotation: 0,
      duration,
      delay,
      ease: 'power3.out',
      scrollTrigger: {
        trigger: el,
        start: 'top 85%',
        toggleActions: 'play none none none',
      },
    })

    return () => {
      st.scrollTrigger?.kill()
      st.kill()
    }
  }, [y, x, scale, rotation, delay, duration])

  return ref
}
