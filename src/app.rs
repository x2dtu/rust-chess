use yew::prelude::*;

use crate::board::BoardComp;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <audio id="ai-move-sound" src="audio/move-self.mp3" preload="auto"></audio>
            <audio id="ai-capture-sound" src="audio/capture.mp3" preload="auto"></audio>
            <audio id="ai-castle-sound" src="audio/castle.mp3" preload="auto"></audio>
            <audio id="ai-check-sound" src="audio/move-check.mp3" preload="auto"></audio>
            <audio id="ai-promote-sound" src="audio/promote.mp3" preload="auto"></audio>
            <audio id="ai-game-over-sound" src="audio/game-end.mp3" preload="auto"></audio>

            <audio id="move-sound" src="audio/move-self.mp3" preload="auto"></audio>
            <audio id="capture-sound" src="audio/capture.mp3" preload="auto"></audio>
            <audio id="castle-sound" src="audio/castle.mp3" preload="auto"></audio>
            <audio id="check-sound" src="audio/move-check.mp3" preload="auto"></audio>
            <audio id="promote-sound" src="audio/promote.mp3" preload="auto"></audio>
            <audio id="game-over-sound" src="audio/game-end.mp3" preload="auto"></audio>
            <BoardComp/>
        </main>
    }
}
