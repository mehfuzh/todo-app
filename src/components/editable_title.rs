use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::app::ProjectName;

#[derive(Properties, PartialEq)]
pub struct EditableTitleProps {
    pub value: ProjectName,
    pub on_change: Callback<ProjectName>,
}

#[function_component(EditableTitle)]
pub fn editable_title(props: &EditableTitleProps) -> Html {
    let editing = use_state(|| false);
    let draft = use_state(|| props.value.text.clone());

    {
        let draft = draft.clone();
        let text = props.value.text.clone();
        use_effect_with(text.clone(), move |t| {
            if *t != *draft {
                draft.set(text.clone());
            }
            ()
        });
    }

    let start = {
        let editing = editing.clone();
        let draft = draft.clone();
        let value = props.value.text.clone();
        Callback::from(move |_: MouseEvent| {
            draft.set(value.clone());
            editing.set(true);
        })
    };

    let cancel = {
        let editing = editing.clone();
        let draft = draft.clone();
        let value = props.value.text.clone();
        Callback::from(move |_: MouseEvent| {
            draft.set(value.clone());
            editing.set(false);
        })
    };

    let on_input = {
        let draft = draft.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target().unwrap().unchecked_into();
            draft.set(input.value());
        })
    };

    let save = {
        let editing = editing.clone();
        let draft = draft.clone();
        let on_change = props.on_change.clone();
        Callback::from(move |_: MouseEvent| {
            let text = (*draft).trim().to_string();
            if !text.is_empty() {
                let next = ProjectName { text };
                on_change.emit(next);
            }
            editing.set(false);
        })
    };

    let on_keydown = {
        let save = save.clone();
        let cancel = cancel.clone();
        Callback::from(move |e: KeyboardEvent| match e.key().as_str() {
            "Enter" => save.emit(MouseEvent::new("click").unwrap()),
            "Escape" => cancel.emit(MouseEvent::new("click").unwrap()),
            _ => {}
        })
    };

    if *editing {
        html! {
            <div class="editable-title editing">
                <input
                    class="editable-title-input"
                    type="text"
                    value={(*draft).clone()}
                    oninput={on_input}
                    onkeydown={on_keydown}
                    maxlength="40"
                />
                <div class="editable-title-actions">
                    <button
                        class="editable-title-btn primary"
                        onclick={save}
                        aria-label="Save name"
                        title="Save"
                    >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <polyline points="20 6 9 17 4 12"></polyline>
                        </svg>
                    </button>
                    <button
                        class="editable-title-btn ghost"
                        onclick={cancel}
                        aria-label="Cancel"
                        title="Cancel"
                    >
                        <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round">
                            <line x1="18" y1="6" x2="6" y2="18"></line>
                            <line x1="6" y1="6" x2="18" y2="18"></line>
                        </svg>
                    </button>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="editable-title">
                <h1 class="app-title">{ &props.value.text }</h1>
                <button
                    class="editable-title-toggle"
                    onclick={start}
                    aria-label={format!("Rename project (currently {})", props.value.text)}
                    title="Rename project"
                >
                    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path>
                        <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path>
                    </svg>
                </button>
            </div>
        }
    }
}
