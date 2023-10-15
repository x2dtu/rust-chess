use crate::square::Square;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct BoardProps {
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

#[function_component(Board)]
pub fn board(props: &BoardProps) -> Html {
    let board_vec = parse_fen(&props.fen);

    html! {
        <div
            class="board"
        >
        { for board_vec.iter().enumerate().map(|(index, piece)| {
            let carry = if (index / 8) % 2 == 1 {1} else {0};
            let color = if (index - carry) % 2 == 0 { "light" } else { "dark" };
            let piece_prop = piece.map(|p| p.to_string()); // Convert Option<&str> to Option<String>
            match piece {
                Some(_) => html!{<Square color={color} piece={piece_prop}/>},
                None => html!{<Square color={color} piece={piece_prop}/>}
            }
        }) }
        </div>
    }
}
