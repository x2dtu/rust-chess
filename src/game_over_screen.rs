use chess::{Color, Game, GameResult};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct GameOverScreenProps {
    pub result: GameResult,
    pub reset_game: Callback<Game>,
}

fn get_string_from_color(color: Color) -> &'static str {
    if color == Color::White {
        "White"
    } else {
        "Black"
    }
}

#[function_component(GameOverScreen)]
pub fn game_over_screen(props: &GameOverScreenProps) -> Html {
    let winning_color = match props.result {
        GameResult::WhiteCheckmates | GameResult::BlackResigns => Some(Color::White),
        GameResult::BlackCheckmates | GameResult::WhiteResigns => Some(Color::Black),
        _ => None,
    };
    let winning_color_str = if winning_color.is_some() {
        Some(get_string_from_color(winning_color.unwrap()))
    } else {
        None
    };
    let winning_message = if winning_color_str.is_some() {
        Some(winning_color_str.unwrap().to_owned() + " Wins!")
    } else {
        None
    };
    let game_message = match props.result {
        GameResult::WhiteCheckmates | GameResult::BlackCheckmates => "Checkmate",
        GameResult::BlackResigns => "Black Resigns",
        GameResult::WhiteResigns => "White Resigns",
        GameResult::DrawAccepted | GameResult::DrawDeclared => "Draw",
        GameResult::Stalemate => "Stalemate",
    };
    let props_copy = props.clone();
    let click_handler = Callback::from(move |_| {
        props_copy.reset_game.emit(Game::new());
    });

    html! {
        <div
        class="game-over"
        >
            <div class="restart-game-modal">
                <div>
                    <p class="game-message">{game_message}</p>
                    {html! {
                        if let Some(winning_message) = winning_message {
                            <p>{winning_message}</p>
                        }
                    }}
                </div>
                <button class="game-restart-button" onclick={click_handler}>{"Play Again?"}</button>
            </div>
        </div>
    }
}
