mod app;
mod board;
pub mod constants;
pub mod evaluation;
mod game_over_screen;
pub mod opening_book;
pub mod search;
mod square;
mod transposition_table;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
