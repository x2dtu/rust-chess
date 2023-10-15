use yew::prelude::*;

use crate::board::BoardComp;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BoardComp fen="rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"/>
        </main>
    }
}
