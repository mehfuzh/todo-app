import { useState, useEffect } from 'react'
import './TodoItem.css'

function formatDueDate(dateStr) {
  if (!dateStr) return null
  const today = new Date()
  today.setHours(0, 0, 0, 0)
  const due = new Date(dateStr + 'T00:00:00')
  const diffDays = Math.round((due - today) / (1000 * 60 * 60 * 24))
  if (diffDays === 0) return { label: 'Today', state: 'today' }
  if (diffDays === 1) return { label: 'Tomorrow', state: 'upcoming' }
  if (diffDays === -1) return { label: 'Yesterday', state: 'overdue' }
  if (diffDays < 0) return { label: `${Math.abs(diffDays)}d overdue`, state: 'overdue' }
  if (diffDays <= 7) return { label: `In ${diffDays}d`, state: 'upcoming' }
  return {
    label: due.toLocaleDateString(undefined, { month: 'short', day: 'numeric' }),
    state: 'upcoming',
  }
}

function TodoItem({ todo, onDelete, onToggle, onToggleEdit, onSaveEdit }) {
  const [editText, setEditText] = useState(todo.text)

  useEffect(() => {
    setEditText(todo.text)
  }, [todo.text])

  const handleSave = () => {
    onSaveEdit(todo.id, editText)
  }

  const due = formatDueDate(todo.dueDate)

  if (todo.isEditing) {
    return (
      <li className={`todo-item editing priority-${todo.priority}`}>
        <input
          type="text"
          className="todo-edit-input"
          value={editText}
          onChange={(e) => setEditText(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === 'Enter') handleSave()
            if (e.key === 'Escape') onToggleEdit(todo.id)
          }}
          autoFocus
        />
        <div className="actions">
          <button className="btn btn-success btn-sm" onClick={handleSave}>
            Save
          </button>
          <button
            className="btn btn-ghost btn-sm"
            onClick={() => onToggleEdit(todo.id)}
          >
            Cancel
          </button>
        </div>
      </li>
    )
  }

  return (
    <li
      className={`todo-item priority-${todo.priority} ${
        todo.completed ? 'completed' : ''
      }`}
    >
      <button
        className={`checkbox ${todo.completed ? 'checked' : ''}`}
        onClick={() => onToggle(todo.id)}
        aria-label={
          todo.completed ? 'Mark as active' : 'Mark as completed'
        }
      >
        {todo.completed && (
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3.5" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="20 6 9 17 4 12" />
          </svg>
        )}
      </button>

      <div className="todo-body">
        <span
          className="todo-text"
          onClick={() => onToggleEdit(todo.id)}
          title="Click to edit"
        >
          {todo.text}
        </span>
        <div className="todo-meta">
          <span className={`priority-badge priority-${todo.priority}`}>
            <span className="priority-dot" />
            {todo.priority}
          </span>
          {due && (
            <span className={`due-badge due-${due.state}`}>
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <rect x="3" y="4" width="18" height="18" rx="2" ry="2" />
                <line x1="16" y1="2" x2="16" y2="6" />
                <line x1="8" y1="2" x2="8" y2="6" />
                <line x1="3" y1="10" x2="21" y2="10" />
              </svg>
              {due.label}
            </span>
          )}
          {todo.githubUrl && (
            <a
              className="github-meta-link"
              href={todo.githubUrl}
              target="_blank"
              rel="noopener noreferrer"
              title={todo.githubUrl}
            >
              <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                <path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" />
                <path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" />
              </svg>
              View on GitHub
            </a>
          )}
        </div>
      </div>

      <div className="actions">
        <button
          className="icon-btn"
          onClick={() => onToggleEdit(todo.id)}
          aria-label="Edit task"
          title="Edit"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" />
            <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" />
          </svg>
        </button>
        <button
          className="icon-btn danger"
          onClick={() => onDelete(todo.id)}
          aria-label="Delete task"
          title="Delete"
        >
          <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <polyline points="3 6 5 6 21 6" />
            <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
            <path d="M10 11v6" />
            <path d="M14 11v6" />
            <path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2" />
          </svg>
        </button>
      </div>
    </li>
  )
}

export default TodoItem
