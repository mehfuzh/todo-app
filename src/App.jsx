import { useState, useEffect } from 'react'
import TodoList from './components/TodoList.jsx'
import TodoStats from './components/TodoStats.jsx'
import ThemeToggle from './components/ThemeToggle.jsx'
import Tabs from './components/Tabs.jsx'
import EditableTitle from './components/EditableTitle.jsx'
import Composer from './components/Composer.jsx'
import Footer from './components/Footer.jsx'
import Welcome from './components/Welcome.jsx'
import './App.css'

const STORAGE_KEY = 'modern-todo-app.v2'
const NAME_STORAGE_KEY = 'modern-todo-app.name'
const DEFAULT_NAME = 'Tada List'
const GITHUB_API = 'https://api.github.com'

function loadTodos() {
  try {
    const raw = localStorage.getItem(STORAGE_KEY)
    if (!raw) return []
    const parsed = JSON.parse(raw)
    return Array.isArray(parsed) ? parsed : []
  } catch {
    return []
  }
}

function loadTheme() {
  try {
    const stored = localStorage.getItem('modern-todo-app.theme')
    if (stored === 'light' || stored === 'dark') return stored
  } catch {}
  if (typeof window !== 'undefined' && window.matchMedia) {
    return window.matchMedia('(prefers-color-scheme: dark)').matches
      ? 'dark'
      : 'light'
  }
  return 'light'
}

function loadName() {
  try {
    const stored = localStorage.getItem(NAME_STORAGE_KEY)
    if (typeof stored === 'string' && stored.trim()) return stored
  } catch {}
  return DEFAULT_NAME
}

function parseGithubUrl(raw) {
  let url = raw.trim()
  if (url.startsWith('github.com/')) url = 'https://' + url
  if (url.startsWith('www.github.com/')) url = 'https://' + url.slice(4)
  if (!/^https?:\/\//i.test(url)) return null
  try {
    const u = new URL(url)
    if (u.hostname !== 'github.com' && u.hostname !== 'www.github.com')
      return null
    const parts = u.pathname.split('/').filter(Boolean)
    if (parts.length < 4) return null
    const kind = parts[2]
    if (kind !== 'issues' && kind !== 'pull') return null
    const number = Number(parts[3])
    if (!Number.isInteger(number) || number <= 0) return null
    return { owner: parts[0], repo: parts[1], number }
  } catch {
    return null
  }
}

async function fetchGithubIssue(parsed, originalUrl) {
  const api = `${GITHUB_API}/repos/${parsed.owner}/${parsed.repo}/issues/${parsed.number}`
  const res = await fetch(api, {
    headers: { Accept: 'application/vnd.github+json' },
  })
  if (res.status === 404) {
    throw new Error('repository or issue is private/missing')
  }
  if (!res.ok) throw new Error(`GitHub returned status ${res.status}`)
  const issue = await res.json()
  const firstLine = (issue.body || '')
    .split('\n')
    .map((l) => l.trim())
    .find((l) => l.length > 0)
  const text = firstLine ? `${issue.title} — ${firstLine}` : issue.title
  return {
    text,
    completed: issue.state === 'closed',
    githubUrl: issue.html_url || originalUrl,
  }
}

const FILTER_TABS = [
  { key: 'all', label: 'All' },
  { key: 'active', label: 'Active' },
  { key: 'completed', label: 'Completed' },
]

function App() {
  const [todos, setTodos] = useState(loadTodos)
  const [theme, setTheme] = useState(loadTheme)
  const [name, setName] = useState(loadName)
  const [filter, setFilter] = useState('all')
  const [draft, setDraft] = useState('')
  const [priority, setPriority] = useState('medium')
  const [dueDate, setDueDate] = useState('')
  const [editingId, setEditingId] = useState(null)
  const [fetching, setFetching] = useState(false)
  const [fetchNotice, setFetchNotice] = useState(null)

  useEffect(() => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(todos))
    } catch {}
  }, [todos])

  useEffect(() => {
    try {
      localStorage.setItem('modern-todo-app.theme', theme)
    } catch {}
    document.documentElement.setAttribute('data-theme', theme)
  }, [theme])

  const changeName = (text) => {
    setName(text)
    try {
      localStorage.setItem(NAME_STORAGE_KEY, text)
    } catch {}
  }

  const addTodo = async () => {
    const text = draft.trim()
    if (!text) return
    setFetchNotice(null)

    const parsed = parseGithubUrl(text)
    if (parsed) {
      setFetching(true)
      let issue = null
      let warning = null
      try {
        issue = await fetchGithubIssue(parsed, text)
      } catch (err) {
        warning = err.message || 'Could not fetch GitHub metadata'
      } finally {
        setFetching(false)
      }

      // Always create the task. If the API fetch failed (e.g. 404 for a
      // private or missing repo), still keep the link on `githubUrl` and
      // use the URL itself as the task text instead of dropping the entry.
      setTodos((prev) => [
        ...prev,
        {
          id: prev.reduce((max, t) => Math.max(max, t.id), 0) + 1,
          text: issue ? issue.text : text,
          completed: issue ? issue.completed : false,
          priority: priority || 'medium',
          dueDate: dueDate || null,
          githubUrl: issue ? issue.githubUrl : text,
          isEditing: false,
        },
      ])
      setDraft('')
      setDueDate('')

      if (warning) {
        setFetchNotice(`Saved with link only — ${warning}.`)
      }
      return
    }

    setTodos((prev) => [
      ...prev,
      {
        id: prev.reduce((max, t) => Math.max(max, t.id), 0) + 1,
        text,
        completed: false,
        priority: priority || 'medium',
        dueDate: dueDate || null,
        isEditing: false,
      },
    ])
    setDraft('')
    setDueDate('')

    // Hide any previously shown fetch notice when a plain task is added.
    setFetchNotice(null)
  }

  const deleteTodo = (id) =>
    setTodos((prev) => prev.filter((t) => t.id !== id))

  const toggleTodo = (id) =>
    setTodos((prev) =>
      prev.map((t) => (t.id === id ? { ...t, completed: !t.completed } : t))
    )

  const toggleEdit = (id) => {
    setEditingId((current) => (current === id ? null : id))
    setTodos((prev) =>
      prev.map((t) => (t.id === id ? { ...t, isEditing: !t.isEditing } : t))
    )
  }

  const saveEdit = (id, text) => {
    const trimmed = (text ?? '').trim()
    if (trimmed) {
      setTodos((prev) =>
        prev.map((t) => (t.id === id ? { ...t, text: trimmed } : t))
      )
    }
    setEditingId(null)
    setTodos((prev) =>
      prev.map((t) => (t.id === id ? { ...t, isEditing: false } : t))
    )
  }

  const visible = todos.filter((t) => {
    if (filter === 'active') return !t.completed
    if (filter === 'completed') return t.completed
    return true
  })

  const total = todos.length
  const completed = todos.filter((t) => t.completed).length
  const active = total - completed
  const hasTodos = total > 0

  return (
    <div className="app">
      <div className="app-container">
        <header className="app-header">
          <div className="header-row">
            <EditableTitle value={name} onChange={changeName} />
            <ThemeToggle
              theme={theme}
              onToggle={() => setTheme(theme === 'dark' ? 'light' : 'dark')}
            />
          </div>
        </header>

        <Composer
          draft={draft}
          setDraft={setDraft}
          priority={priority}
          setPriority={setPriority}
          dueDate={dueDate}
          setDueDate={setDueDate}
          fetching={fetching}
          fetchNotice={fetchNotice}
          onAdd={addTodo}
        />

        {hasTodos && (
          <TodoStats total={total} completed={completed} active={active} />
        )}

        {hasTodos && (
          <>
            <div className="toolbar">
              <Tabs
                tabs={FILTER_TABS}
                active={filter}
                onChange={(key) => setFilter(key)}
              />
            </div>

            <TodoList
              todos={visible}
              onDelete={deleteTodo}
              onToggle={toggleTodo}
              onToggleEdit={toggleEdit}
              onSaveEdit={saveEdit}
            />

            <Footer shown={visible.length} total={total} />
          </>
        )}

        {!hasTodos && <Welcome />}
      </div>
    </div>
  )
}

export default App
