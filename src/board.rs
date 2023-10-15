use std::collections::HashSet;
use std::str::FromStr;

use crate::square::SquareComp;
use chess::{Board, MoveGen, Square};
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BoardCompProps {
    pub fen: String,
}

fn parse_fen(fen: &str) -> Vec<Option<&str>> {
    let mut result = Vec::new();

    // Split the FEN string by spaces and get the piece placement part
    let fen_parts: Vec<&str> = fen.split(' ').collect();
    if let Some(piece_placement) = fen_parts.get(0) {
        // Iterate over the characters in the piece placement part
        for c in piece_placement.chars() {
            match c {
                'r' => result.push(Some("img/72x72blackrook.png")),
                'n' => result.push(Some("img/72x72blackknight.png")),
                'b' => result.push(Some("img/72x72blackbishop.png")),
                'q' => result.push(Some("img/72x72blackqueen.png")),
                'k' => result.push(Some("img/72x72blackking.png")),
                'p' => result.push(Some("img/72x72blackpawn.png")),
                'R' => result.push(Some("img/72x72rook.png")),
                'N' => result.push(Some("img/72x72knight.png")),
                'B' => result.push(Some("img/72x72bishop.png")),
                'Q' => result.push(Some("img/72x72queen.png")),
                'K' => result.push(Some("img/72x72king.png")),
                'P' => result.push(Some("img/72x72pawn.png")),
                '1'..='8' => {
                    let num = c.to_digit(10).unwrap_or(1) as usize;
                    for _ in 0..num {
                        result.push(None);
                    }
                }
                _ => {}
            }
        }
    }

    result
}

#[function_component(BoardComp)]
pub fn board(props: &BoardCompProps) -> Html {
    let board_vec = parse_fen(&props.fen);
    let selected = use_state(|| None);
    let set_selected = {
        let selected = selected.clone();
        Callback::from(move |new_selected| selected.set(new_selected))
    };
    let board = Board::from_str(&props.fen).unwrap();
    let mut moves = HashSet::new();

    if selected.is_some() {
        let square: Square = selected.unwrap();
        for legal_move in MoveGen::new_legal(&board) {
            if legal_move.get_source() == square {
                moves.insert(legal_move.get_dest());
            }
        }
    }

    html! {
        <div
            class="board"
        >
        { for board_vec.iter().enumerate().map(|(index, piece)| {
            let carry = if (index / 8) % 2 == 1 {1} else {0};
            let color = if (index - carry) % 2 == 0 { "light" } else { "dark" };
            let piece_prop = piece.map(|p| p.to_string()); // Convert Option<&str> to Option<String>

            // get the square
            let file_index = index % 8;
            let rank_index = 7 - (index / 8);
            let algebraic = format!(
                "{}{}",
                (file_index + 'a' as usize) as u8 as char,
                (rank_index + '1' as usize) as u8 as char
            );
            let square = Square::from_str(&algebraic).unwrap();
            let can_move_to = moves.contains(&square);

            match piece {
                Some(_) => html!{<SquareComp color={color} piece={piece_prop} set_selected={set_selected.clone()} can_move_to={can_move_to} square={square}/>},
                None => html!{<SquareComp color={color} piece={piece_prop} set_selected={set_selected.clone()} can_move_to={can_move_to} square={square}/>}
            }
        }) }
        </div>
    }
}
// bit_square={bit_square} selected={selected_local} can_move_to={false}
