# Tada List

A small todo app built two ways from the same project: a React/JSX frontend (the primary, Vite-served build) and a Yew/Rust frontend that compiles to WebAssembly. Both share the same component shapes, styles, and local-first behavior.

## Features

- Add tasks with a **priority** (low / medium / high) and a **due date**
- Paste a GitHub issue or pull request link to import its title, body, and completion state as a task
- Mark tasks complete, edit them inline, or delete them
- Filter by **All / Active / Completed**
- Light / dark theme (auto-detects `prefers-color-scheme`, persists choice)
- Renameable project title (persisted)
- Everything stored in `localStorage` — no backend, no accounts

## Project layout

