use yew::prelude::*;

use crate::board::BoardComp;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <BoardComp/>
        </main>
    }
}
