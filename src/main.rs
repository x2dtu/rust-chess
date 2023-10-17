mod app;
mod board;
pub mod bot;
pub mod constants;
pub mod evaluation;
mod game_over_screen;
pub mod opening_book;
mod square;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
