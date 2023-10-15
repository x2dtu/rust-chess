use std::collections::HashSet;
use std::str::FromStr;
// use web_sys::File;

use crate::{bot::choose_move, opening_book::opening_book_move, square::SquareComp};
use chess::{Board, ChessMove, Color, File, MoveGen, Piece, Rank, Square};
use yew::prelude::*;

fn parse_board(board: &Board) -> Vec<Option<&str>> {
    let mut result = Vec::new();

    // Split the FEN string by spaces and get the piece placement part
    for rank in (0..8).rev() {
        for file in 0..8 {
            let mut c = '0';
            let square = Square::make_square(Rank::from_index(rank), File::from_index(file));
            if board.piece_on(square).is_some() {
                let p = board.piece_on(square).unwrap();
                c = p
                    .to_string(board.color_on(square).unwrap())
                    .chars()
                    .next()
                    .unwrap();
            }
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
                _ => result.push(None),
            }
        }
    }
    result
}

#[function_component(BoardComp)]
pub fn board() -> Html {
    let board = use_state(|| Board::default());
    let selected = use_state(|| None);
    let target = use_state(|| None);
    let in_opening_book = use_state(|| false);
    let set_selected = {
        let selected = selected.clone();
        Callback::from(move |new_selected| selected.set(new_selected))
    };
    let set_target = {
        let target = target.clone();
        Callback::from(move |new_target| target.set(new_target))
    };
    if (*target).is_some() && (*selected).is_some() {
        let mut new_move = ChessMove::new(selected.unwrap(), target.unwrap(), None);
        if target.unwrap().get_rank() == Rank::Eighth
            && board.piece_on(selected.unwrap()).unwrap() == Piece::Pawn
        {
            // then we are promoting a pawn. lets just auto queen for now
            new_move = ChessMove::new(selected.unwrap(), target.unwrap(), Some(Piece::Queen));
        }
        let mut board_copy = (*board).clone();
        board.make_move(new_move, &mut board_copy);
        board.set(board_copy);

        // unset the move and selection
        selected.set(None);
        target.set(None);
    } else if board.side_to_move() == Color::Black {
        if *in_opening_book {
            let ai_move = opening_book_move(board.get_hash());
            if ai_move.is_some() {
                let ai_move = ai_move.unwrap();
                let mut board_copy = (*board).clone();
                board.make_move(ai_move, &mut board_copy);
                board.set(board_copy);
            } else {
                // we just got out of opening book, so choose a move on our own now
                in_opening_book.set(false);
                let ai_move = choose_move(&board);
                if ai_move.is_some() {
                    let ai_move = ai_move.unwrap();
                    let mut board_copy = (*board).clone();
                    board.make_move(ai_move, &mut board_copy);
                    board.set(board_copy);
                }
            }
        } else {
            let ai_move = choose_move(&board);
            if ai_move.is_some() {
                let ai_move = ai_move.unwrap();
                let mut board_copy = (*board).clone();
                board.make_move(ai_move, &mut board_copy);
                board.set(board_copy);
            }
        }
    }

    let board_vec = parse_board(&board);
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
                Some(_) => html!{<SquareComp color={color} piece={piece_prop} set_selected={set_selected.clone()} set_target={set_target.clone()} can_move_to={can_move_to} square={square}/>},
                None => html!{<SquareComp color={color} piece={piece_prop} set_selected={set_selected.clone()} set_target={set_target.clone()} can_move_to={can_move_to} square={square}/>}
            }
        }) }
        </div>
    }
}
