use chess::GameResult;
// use gloo_console::log;
// use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GameOverScreenProps {
    pub result: GameResult,
}

#[function_component(GameOverScreen)]
pub fn game_over_screen(props: &GameOverScreenProps) -> Html {
    html! {
        <div
        class="game-over"
        >
       {"Game Over"}
        </div>
    }
}
