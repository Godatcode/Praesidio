import { useEffect } from 'react'

// iOS Safari's native smooth scroll (html { scroll-behavior: smooth }) can stop partway
// through long in-page jumps, e.g. the hero CTA -> #cta landing on the comparison table.
// Drive in-page anchor scrolls from JS and re-scroll until the target actually reaches the top.
const SETTLE_MS = 150
const TOLERANCE_PX = 4
const MAX_CORRECTIONS = 3

export function useAnchorScroll() {
  useEffect(() => {
    let cancel: (() => void) | null = null

    const scrollToTarget = (target: HTMLElement, behavior: ScrollBehavior = 'smooth') => {
      cancel?.()
      let timer = 0
      let lastY = -1
      let corrections = 0

      // The visitor scrolling by hand takes over; stop correcting
      const stop = () => {
        window.clearTimeout(timer)
        window.removeEventListener('touchstart', stop)
        window.removeEventListener('wheel', stop)
        cancel = null
      }
      cancel = stop
      window.addEventListener('touchstart', stop, { passive: true })
      window.addEventListener('wheel', stop, { passive: true })

      const scroll = () => target.scrollIntoView({ behavior, block: 'start' })

      const check = () => {
        const y = window.scrollY
        if (y !== lastY) {
          lastY = y
          timer = window.setTimeout(check, SETTLE_MS)
          return
        }
        const reached = Math.abs(target.getBoundingClientRect().top) <= TOLERANCE_PX
        const atBottom = window.innerHeight + y >= document.documentElement.scrollHeight - TOLERANCE_PX
        if (reached || atBottom || corrections >= MAX_CORRECTIONS) {
          stop()
          return
        }
        corrections++
        lastY = -1
        scroll()
        timer = window.setTimeout(check, SETTLE_MS)
      }

      scroll()
      timer = window.setTimeout(check, SETTLE_MS)
    }

    const onClick = (e: MouseEvent) => {
      if (e.defaultPrevented || e.button !== 0 || e.metaKey || e.ctrlKey || e.shiftKey || e.altKey) return
      const link = (e.target as Element | null)?.closest?.('a[href^="#"]')
      const id = link?.getAttribute('href')?.slice(1)
      const target = id ? document.getElementById(id) : null
      if (!target) return
      e.preventDefault()
      history.pushState(null, '', `#${id}`)
      scrollToTarget(target)
    }

    // A shared link such as praesidio.live/#cta: the browser looks for the #hash target while
    // parsing index.html, before React has rendered any section, so it never scrolls. Jump to it
    // once the sections exist.
    const frame = window.requestAnimationFrame(() => {
      const id = decodeURIComponent(window.location.hash.slice(1))
      const target = id ? document.getElementById(id) : null
      if (target) scrollToTarget(target, 'instant')
    })

    document.addEventListener('click', onClick)
    return () => {
      window.cancelAnimationFrame(frame)
      document.removeEventListener('click', onClick)
      cancel?.()
    }
  }, [])
}
