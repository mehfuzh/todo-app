use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::TodoItemData;

#[derive(Properties, PartialEq)]
pub struct TodoItemProps {
    pub item: TodoItemData,
    pub on_toggle: Callback<()>,
    pub on_delete: Callback<()>,
    pub on_edit: Callback<String>,
}

#[function_component(TodoItem)]
pub fn todo_item(props: &TodoItemProps) -> Html {
    let editing = use_state(|| false);
    let draft = use_state(|| props.item.text.clone());

    // Reset draft when the underlying item text changes externally
    {
        let draft = draft.clone();
        let text = props.item.text.clone();
        use_effect_with(props.item.id, move |_| {
            draft.set(text);
            ()
        });
    }

    let on_check = {
        let on_toggle = props.on_toggle.clone();
        Callback::from(move |_: MouseEvent| on_toggle.emit(()))
    };

    let on_delete = {
        let on_delete = props.on_delete.clone();
        Callback::from(move |_: MouseEvent| on_delete.emit(()))
    };

    let on_edit_click = {
        let editing = editing.clone();
        Callback::from(move |_: MouseEvent| editing.set(true))
    };

    let on_input = {
        let draft = draft.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().unchecked_into();
            draft.set(input.value());
        })
    };

    let do_save = {
        let editing = editing.clone();
        let draft = draft.clone();
        let on_edit = props.on_edit.clone();
        Callback::from(move |_: MouseEvent| {
            let text = (*draft).trim().to_string();
            if !text.is_empty() {
                on_edit.emit(text);
            }
            editing.set(false);
        })
    };

    let on_cancel = {
        let editing = editing.clone();
        let draft = draft.clone();
        let original = props.item.text.clone();
        Callback::from(move |_: MouseEvent| {
            draft.set(original.clone());
            editing.set(false);
        })
    };

    let on_keydown = {
        let do_save = do_save.clone();
        let on_cancel = on_cancel.clone();
        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "Enter" => do_save.emit(MouseEvent::new("click").unwrap()),
                "Escape" => on_cancel.emit(MouseEvent::new("click").unwrap()),
                _ => {}
            }
        })
    };

    let class = if props.item.completed {
        "todo-item completed"
    } else {
        "todo-item"
    };

    html! {
        <li class={class}>
            <button
                class={if props.item.completed { "checkbox checked" } else { "checkbox" }}
                onclick={on_check}
                aria-label={if props.item.completed { "Mark as active" } else { "Mark as completed" }}
            >
                if props.item.completed {
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round">
                        <polyline points="20 6 9 17 4 12"></polyline>
                    </svg>
                }
            </button>

            if *editing {
                <input
                    class="todo-edit-input"
                    type="text"
                    value={(*draft).clone()}
                    oninput={on_input}
                    onkeydown={on_keydown}
                />
            } else {
                <span class="todo-text" onclick={on_edit_click} title="Click to edit">
                    { &props.item.text }
                </span>
            }

            <button class="delete-btn" onclick={on_delete} aria-label="Delete task">
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <polyline points="3 6 5 6 21 6"></polyline>
                    <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6"></path>
                    <path d="M10 11v6"></path>
                    <path d="M14 11v6"></path>
                    <path d="M9 6V4a2 2 0 0 1 2-2h2a2 2 0 0 1 2 2v2"></path>
                </svg>
            </button>
        </li>
    }
}
