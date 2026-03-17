import { useEffect, useRef } from 'react'
import gsap from 'gsap'
import { ScrollTrigger } from 'gsap/ScrollTrigger'
import SplitType from 'split-type'

gsap.registerPlugin(ScrollTrigger)

export function useSplitTextReveal(type: 'words' | 'chars' = 'words') {
  const ref = useRef<HTMLElement>(null)

  useEffect(() => {
    const el = ref.current
    if (!el) return

    const split = new SplitType(el, { types: type })
    const targets = type === 'words' ? split.words : split.chars

    if (!targets || targets.length === 0) return

    gsap.set(targets, {
      opacity: 0,
      y: 20,
      rotateX: -10,
    })

    gsap.to(targets, {
      opacity: 1,
      y: 0,
      rotateX: 0,
      duration: 0.6,
      ease: 'power3.out',
      stagger: type === 'words' ? 0.04 : 0.02,
      scrollTrigger: {
        trigger: el,
        start: 'top 85%',
        end: 'top 20%',
        toggleActions: 'play none none none',
      },
    })

    return () => {
      split.revert()
      ScrollTrigger.getAll().forEach((st) => {
        if (st.trigger === el) st.kill()
      })
    }
  }, [type])

  return ref
}
