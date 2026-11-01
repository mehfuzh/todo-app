import { useRef, useEffect, useState } from 'react'
import './Tabs.css'

function Tabs({ tabs, active, onChange }) {
  const containerRef = useRef(null)
  const [slider, setSlider] = useState({ left: 0, width: 0 })

  useEffect(() => {
    const container = containerRef.current
    if (!container) return

    const activeIndex = tabs.findIndex((t) => t.key === active)
    if (activeIndex === -1) return

    const buttons = container.querySelectorAll('.tabs-btn')
    const activeButton = buttons[activeIndex]
    if (!activeButton) return

    const update = () => {
      const cRect = container.getBoundingClientRect()
      const bRect = activeButton.getBoundingClientRect()
      setSlider({ left: bRect.left - cRect.left, width: bRect.width })
    }

    update()
    window.addEventListener('resize', update)
    return () => window.removeEventListener('resize', update)
  }, [active, tabs])

  return (
    <div
      className="tabs"
      role="tablist"
      data-active={active}
      ref={containerRef}
    >
      <span
        className="tabs-slider"
        aria-hidden="true"
        style={{ left: slider.left, width: slider.width }}
      />
      {tabs.map((tab) => (
        <button
          key={tab.key}
          role="tab"
          aria-selected={active === tab.key}
          className={`tabs-btn ${active === tab.key ? 'active' : ''}`}
          onClick={() => onChange(tab.key)}
          type="button"
        >
          {tab.label}
        </button>
      ))}
    </div>
  )
}

export default Tabs
