import './TodoStats.css'

function TodoStats({ total, completed, active }) {
  const pct = total === 0 ? 0 : Math.round((completed / total) * 100)
  const radius = 26
  const circumference = 2 * Math.PI * radius
  const offset = circumference - (pct / 100) * circumference

  return (
    <div className="todo-stats">
      <div className="progress-ring">
        <svg width="68" height="68" viewBox="0 0 68 68">
          <defs>
            <linearGradient
              id="statsProgressGradient"
              x1="0"
              y1="0"
              x2="1"
              y2="1"
            >
              <stop offset="0%" stopColor="#6366f1" />
              <stop offset="100%" stopColor="#a855f7" />
            </linearGradient>
          </defs>
          <circle
            cx="34"
            cy="34"
            r={radius}
            fill="none"
            className="ring-track"
          />
          <circle
            cx="34"
            cy="34"
            r={radius}
            fill="none"
            className="ring-fill"
            strokeDasharray={circumference}
            strokeDashoffset={offset}
            transform="rotate(-90 34 34)"
          />
        </svg>
        <span className="progress-label">
          <span className="progress-number">{pct}</span>
          <span className="progress-percent">%</span>
        </span>
      </div>

      <div className="stats-list">
        <div className="stat-row">
          <span className="stat-label">Total</span>
          <span className="stat-value">{total}</span>
        </div>
        <div className="stat-row">
          <span className="stat-label">Active</span>
          <span className="stat-value">{active}</span>
        </div>
        <div className="stat-row">
          <span className="stat-label">Completed</span>
          <span className="stat-value">{completed}</span>
        </div>
      </div>
    </div>
  )
}

export default TodoStats
