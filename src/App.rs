use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

use crate::components::editable_title::EditableTitle;
use crate::components::github_link_input::GithubLinkInput;
use crate::components::todo_list::TodoList;

const STORAGE_KEY: &str = "modern-todo-app.todos";
const NAME_STORAGE_KEY: &str = "modern-todo-app.name";
const DEFAULT_NAME: &str = "Tada List";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TodoItemData {
    pub id: u32,
    pub text: String,
    pub completed: bool,
    // Added for GitHub link support
    pub github_url: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectName {
    pub text: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TodoListProps {
    pub items: Vec<TodoItemData>,
    pub on_toggle: Callback<u32>,
    pub on_delete: Callback<u32>,
    pub on_edit: Callback<(u32, String)>,
}

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(|| {
        LocalStorage::get::<Vec<TodoItemData>>(STORAGE_KEY).unwrap_or_default()
    });

    let filter = use_state(|| Filter::All);

    let project_name = use_state(|| ProjectName {
        text: LocalStorage::get::<String>(NAME_STORAGE_KEY)
            .unwrap_or_else(|_| DEFAULT_NAME.to_string()),
    });

    // Persist the todo list whenever it changes.
    use_effect_with((*todos).clone(), |items: &Vec<TodoItemData>| {
        let _ = LocalStorage::set(STORAGE_KEY, items.clone());
        ()
    });

    // A task created from a GitHub link; a fresh id is assigned on append.
    let on_create = {
        let todos = todos.clone();
        Callback::from(move |mut item: TodoItemData| {
            let mut next = (*todos).clone();
            item.id = next.iter().map(|i| i.id).max().unwrap_or(0) + 1;
            next.push(item);
            todos.set(next);
        })
    };

    let on_toggle = {
        let todos = todos.clone();
        Callback::from(move |id: u32| {
            let mut next = (*todos).clone();
            if let Some(item) = next.iter_mut().find(|i| i.id == id) {
                item.completed = !item.completed;
            }
            todos.set(next);
        })
    };

    let on_delete = {
        let todos = todos.clone();
        Callback::from(move |id: u32| {
            let next: Vec<TodoItemData> = (*todos).iter().filter(|i| i.id != id).cloned().collect();
            todos.set(next);
        })
    };

    let on_edit = {
        let todos = todos.clone();
        Callback::from(move |(id, text): (u32, String)| {
            let mut next = (*todos).clone();
            if let Some(item) = next.iter_mut().find(|i| i.id == id) {
                item.text = text;
            }
            todos.set(next);
        })
    };

    let on_name_change = {
        let project_name = project_name.clone();
        Callback::from(move |name: ProjectName| {
            let _ = LocalStorage::set(NAME_STORAGE_KEY, name.text.clone());
            project_name.set(name);
        })
    };

    let on_filter_all = {
        let filter = filter.clone();
        Callback::from(move |_: MouseEvent| filter.set(Filter::All))
    };
    let on_filter_active = {
        let filter = filter.clone();
        Callback::from(move |_: MouseEvent| filter.set(Filter::Active))
    };
    let on_filter_completed = {
        let filter = filter.clone();
        Callback::from(move |_: MouseEvent| filter.set(Filter::Completed))
    };

    let visible: Vec<TodoItemData> = todos
        .iter()
        .filter(|item| match *filter {
            Filter::All => true,
            Filter::Active => !item.completed,
            Filter::Completed => item.completed,
        })
        .cloned()
        .collect();

    let total = todos.len();
    let completed_count = todos.iter().filter(|item| item.completed).count();
    let active_count = total - completed_count;
    // Capture the count before `visible` is moved into the props below.
    let shown_count = visible.len();

    html! {
        <div class="app-shell">
            <div class="app-container">
                <header class="app-header">
                    <EditableTitle value={(*project_name).clone()} on_change={on_name_change} />
                    <p class="app-subtitle">
                        { "Paste a GitHub issue or pull request link to add an item to your list." }
                    </p>
                </header>

                <section class="composer">
                    <GithubLinkInput on_create={on_create} />
                </section>

                if total > 0 {
                    <div class="filter-group" role="group" aria-label="Filter items">
                        <button
                            class={if *filter == Filter::All { "filter-btn active" } else { "filter-btn" }}
                            onclick={on_filter_all}
                        >
                            { "All" }
                        </button>
                        <button
                            class={if *filter == Filter::Active { "filter-btn active" } else { "filter-btn" }}
                            onclick={on_filter_active}
                        >
                            { "Active" }
                        </button>
                        <button
                            class={if *filter == Filter::Completed { "filter-btn active" } else { "filter-btn" }}
                            onclick={on_filter_completed}
                        >
                            { "Completed" }
                        </button>
                    </div>

                    <TodoList
                        items={visible}
                        on_toggle={on_toggle}
                        on_delete={on_delete}
                        on_edit={on_edit}
                    />

                    <footer class="app-footer">
                        <div class="footer-stats">
                            <span class="stat-pill">{ format!("{} total", total) }</span>
                            <span class="stat-pill subtle">{ format!("{} active", active_count) }</span>
                            <span class="stat-pill">{ format!("{} completed", completed_count) }</span>
                        </div>
                        <span class="empty-text">
                            { format!("{} of {} shown", shown_count, total) }
                        </span>
                    </footer>
                } else {
                    <div class="welcome-screen">
                        <p class="welcome-text">{ "Add your first item to get started." }</p>
                    </div>
                }
            </div>
        </div>
    }
}
