import { useEffect, useState } from 'react'
import { useSpring, useMotionValue } from 'framer-motion'

interface AnimatedNumberProps {
  value: number
  className?: string
  format?: (n: number) => string
}

export function AnimatedNumber({ value, className = 'metric-value', format }: AnimatedNumberProps) {
  const [displayValue, setDisplayValue] = useState(0)
  const motionValue = useMotionValue(0)
  const spring = useSpring(motionValue, { duration: 1200, bounce: 0 })

  useEffect(() => {
    motionValue.set(typeof value === 'number' && !isNaN(value) ? value : 0)
  }, [value, motionValue])

  useEffect(() => {
    const unsubscribe = spring.on('change', (v) => {
      setDisplayValue(Math.round(v))
    })
    return unsubscribe
  }, [spring])

  const text = format ? format(displayValue) : displayValue.toString()

  return <span className={className}>{text}</span>
}
