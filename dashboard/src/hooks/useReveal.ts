import { useCallback, useEffect, useRef } from 'react'

/**
 * IntersectionObserver-based scroll reveal.
 * Uses a callback ref so it works even when the element
 * mounts after the initial render (e.g. after data loads).
 */
export function useReveal<T extends HTMLElement = HTMLDivElement>() {
  const observerRef = useRef<IntersectionObserver | null>(null)

  // Create observer once
  if (!observerRef.current && typeof IntersectionObserver !== 'undefined') {
    observerRef.current = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('revealed')
            observerRef.current?.unobserve(entry.target)
          }
        })
      },
      { threshold: 0.05, rootMargin: '0px 0px -50px 0px' },
    )
  }

  const ref = useCallback(
    (node: T | null) => {
      if (node && observerRef.current) {
        observerRef.current.observe(node)
        // Force check immediately in case element is already in view
        requestAnimationFrame(() => {
          const rect = node.getBoundingClientRect()
          const isInView = rect.top < window.innerHeight && rect.bottom > 0
          if (isInView) {
            node.classList.add('revealed')
          }
        })
      }
    },
    [],
  )

  useEffect(() => {
    return () => observerRef.current?.disconnect()
  }, [])

  return ref
}

/**
 * Observe all children with .reveal class inside a container.
 */
export function useRevealChildren<T extends HTMLElement = HTMLDivElement>() {
  const ref = useRef<T>(null)
  const observerRef = useRef<IntersectionObserver | null>(null)

  useEffect(() => {
    const container = ref.current
    if (!container) return

    observerRef.current = new IntersectionObserver(
      (entries) => {
        entries.forEach((entry) => {
          if (entry.isIntersecting) {
            entry.target.classList.add('revealed')
            observerRef.current?.unobserve(entry.target)
          }
        })
      },
      { threshold: 0.05, rootMargin: '0px 0px -50px 0px' },
    )

    const children = container.querySelectorAll('.reveal')
    children.forEach((child) => {
      observerRef.current?.observe(child)
      // Force check immediately for elements already in view
      requestAnimationFrame(() => {
        const rect = child.getBoundingClientRect()
        const isInView = rect.top < window.innerHeight && rect.bottom > 0
        if (isInView) {
          child.classList.add('revealed')
        }
      })
    })

    return () => observerRef.current?.disconnect()
  })

  return ref
}
