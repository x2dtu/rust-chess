mod app;
mod board;
pub mod bot;
pub mod constants;
mod square;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
