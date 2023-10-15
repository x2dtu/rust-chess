mod app;
mod board;
mod square;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
