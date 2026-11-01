import TodoItem from './TodoItem.jsx'
import './TodoList.css'

function TodoList({ todos, onDelete, onToggle, onToggleEdit, onSaveEdit }) {
  if (todos.length === 0) {
    return (
      <div className="empty-state">
        <div className="empty-illustration" aria-hidden="true">
          <svg width="56" height="56" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round">
            <path d="M9 11l3 3L22 4" />
            <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" />
          </svg>
        </div>
        <h3 className="empty-title">All caught up</h3>
        <p className="empty-text">
          Add a task above to get started.
        </p>
      </div>
    )
  }

  return (
    <ul className="todo-list">
      {todos.map((todo) => (
        <TodoItem
          key={todo.id}
          todo={todo}
          onDelete={onDelete}
          onToggle={onToggle}
          onToggleEdit={onToggleEdit}
          onSaveEdit={onSaveEdit}
        />
      ))}
    </ul>
  )
}

export default TodoList
