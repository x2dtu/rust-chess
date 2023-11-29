pub mod constants;
pub mod evaluation;
pub mod move_orderer;
pub mod opening_book;
pub mod search;
mod transposition_table;
pub mod wasm;

use wasm::app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
