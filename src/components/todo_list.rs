use yew::prelude::*;

use crate::app::TodoListProps;
use crate::components::todo_item::TodoItem;

#[function_component(TodoList)]
pub fn todo_list(props: &TodoListProps) -> Html {
    if props.items.is_empty() {
        return html! {
            <div class="empty-state">
                <div class="empty-illustration" aria-hidden="true">
                    <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M9 11l3 3L22 4"></path>
                        <path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"></path>
                    </svg>
                </div>
                <h3 class="empty-title">{ "All caught up" }</h3>
                <p class="empty-text">{ "Add a task above to get started." }</p>
            </div>
        };
    }

    html! {
        <ul class="todo-list">
            { for props.items.iter().map(|item| {
                let on_toggle = {
                    let cb = props.on_toggle.clone();
                    let id = item.id;
                    Callback::from(move |_: ()| cb.emit(id))
                };
                let on_delete = {
                    let cb = props.on_delete.clone();
                    let id = item.id;
                    Callback::from(move |_: ()| cb.emit(id))
                };
                let on_edit = {
                    let cb = props.on_edit.clone();
                    let id = item.id;
                    Callback::from(move |text: String| cb.emit((id, text)))
                };

                html! {
                    <TodoItem
                        key={item.id}
                        item={item.clone()}
                        on_toggle={on_toggle}
                        on_delete={on_delete}
                        on_edit={on_edit}
                    />
                }
            }) }
        </ul>
    }
}
