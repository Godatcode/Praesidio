import { useEffect, useRef } from 'react'

interface RevealOptions {
  y?: number
  delay?: number
  scale?: number
}

export function useLandingReveal(options: RevealOptions = {}) {
  const ref = useRef<HTMLDivElement>(null)
  const { y = 30, delay = 0, scale } = options

  useEffect(() => {
    const el = ref.current
    if (!el) return

    el.style.opacity = '0'
    el.style.transform = `translateY(${y}px)${scale ? ` scale(${scale})` : ''}`
    el.style.transition = `opacity 0.7s cubic-bezier(0.16, 1, 0.3, 1) ${delay}s, transform 0.7s cubic-bezier(0.16, 1, 0.3, 1) ${delay}s`

    const obs = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          el.style.opacity = '1'
          el.style.transform = 'translateY(0) scale(1)'
          obs.disconnect()
        }
      },
      { threshold: 0.1 },
    )
    obs.observe(el)
    return () => obs.disconnect()
  }, [y, delay, scale])

  return ref
}

export function useStaggerReveal(selector: string, staggerMs = 100) {
  const ref = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const container = ref.current
    if (!container) return

    const items = container.querySelectorAll<HTMLElement>(selector)
    items.forEach((el, i) => {
      el.style.opacity = '0'
      el.style.transform = 'translateY(30px)'
      el.style.transition = `opacity 0.6s cubic-bezier(0.16, 1, 0.3, 1) ${i * staggerMs}ms, transform 0.6s cubic-bezier(0.16, 1, 0.3, 1) ${i * staggerMs}ms`
    })

    const obs = new IntersectionObserver(
      ([entry]) => {
        if (entry.isIntersecting) {
          items.forEach((el) => {
            el.style.opacity = '1'
            el.style.transform = 'translateY(0)'
          })
          obs.disconnect()
        }
      },
      { threshold: 0.1 },
    )
    obs.observe(container)
    return () => obs.disconnect()
  }, [selector, staggerMs])

  return ref
}
