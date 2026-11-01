#[path = "app.rs"]
mod app;

#[path = "components/mod.rs"]
mod components;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
