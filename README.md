# Tada List

> This project was generated using [docent](https://github.com/smartloop-ai/docent) and lives under the [smartloop-ai](https://github.com/smartloop-ai) organization.

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

The app has two parallel frontends built from the same project root, sharing a single `src/` tree:

- **React (primary)** — JSX under `src/`, built and served by Vite. Entry: `src/main.jsx` → `src/App.jsx`.
- **Yew / Rust** — Rust under `src/`, compiled to WebAssembly. Entry: `src/main.rs` → `src/App.rs`.

## Repository

- Organization: `smartloop-ai`
- Remote: `git@github.com:smartloop-ai/todo-app.git`
