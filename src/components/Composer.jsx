import './Composer.css'

const PRIORITIES = ['low', 'medium', 'high']

function Composer({
  draft,
  setDraft,
  priority,
  setPriority,
  dueDate,
  setDueDate,
  fetching,
  fetchNotice,
  onAdd,
}) {
  const handleKeyDown = (e) => {
    if (e.key === 'Enter' && !fetching) onAdd()
  }

  return (
    <section className="composer">
      <input
        type="text"
        className="composer-input"
        placeholder="Paste a GitHub issue or pull request link to get started"
        value={draft}
        onChange={(e) => setDraft(e.target.value)}
        onKeyDown={handleKeyDown}
        disabled={fetching}
      />
      <div className="composer-options">
        <div className="priority-group" role="group" aria-label="Priority">
          {PRIORITIES.map((level) => (
            <button
              key={level}
              type="button"
              className={`priority-chip priority-${level} ${
                priority === level ? 'active' : ''
              }`}
              onClick={() => setPriority(level)}
            >
              <span className="priority-dot" />
              {level}
            </button>
          ))}
        </div>
        <input
          type="date"
          className="date-input"
          value={dueDate}
          onChange={(e) => setDueDate(e.target.value)}
          aria-label="Due date"
        />
        <button
          type="button"
          className="btn btn-primary"
          onClick={onAdd}
          disabled={!draft.trim() || fetching}
        >
          {fetching ? 'Fetching…' : 'Add'}
        </button>
      </div>
      {fetchNotice && (
        <p className="composer-notice" role="status">
          {fetchNotice}
        </p>
      )}
    </section>
  )
}

export default Composer
