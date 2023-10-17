use gloo_timers::callback::Timeout;
use std::collections::HashSet;
use std::str::FromStr;
use wasm_bindgen::JsCast;

use crate::{
    bot::choose_move, game_over_screen::GameOverScreen, opening_book::opening_book_move,
    square::SquareComp,
};
use chess::{Board, BoardStatus, ChessMove, Color, File, Game, MoveGen, Piece, Rank, Square};
use web_sys::HtmlAudioElement;
use yew::prelude::*;

fn play_move_sound(board: &Board, chess_move: &ChessMove, is_ai: bool) {
    let is_capture = is_move_a_capture(board, chess_move);
    let mut board_after_move = board.clone();
    board.make_move(*chess_move, &mut board_after_move);

    let is_check = board_after_move.checkers().popcnt() > 0;
    let is_promotion = chess_move.get_promotion().is_some();
    let is_castle = is_move_a_castle(board, chess_move);
    let game_over = board_after_move.status() != BoardStatus::Ongoing;

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let sound = if game_over {
        "game-over-sound"
    } else if is_check {
        "check-sound"
    } else if is_capture {
        "capture-sound"
    } else if is_promotion {
        "promote-sound"
    } else if is_castle {
        "castle-sound"
    } else {
        "move-sound"
    };
    let mut player = if is_ai {
        "ai-".to_owned()
    } else {
        "".to_owned()
    };
    player.push_str(sound);
    let audio = document
        .get_element_by_id(&player)
        .expect("should have an audio element");

    let audio: HtmlAudioElement = audio
        .dyn_into::<HtmlAudioElement>()
        .expect("element should be an audio element");

    let _ = audio.play().expect("failed to play audio");
}

fn is_move_a_capture(board: &Board, chess_move: &ChessMove) -> bool {
    let target_square = chess_move.get_dest();
    let source_square = chess_move.get_source();
    if board.piece_on(target_square).is_some() {
        return true;
    }
    if let Some(en_passant_square) = board.en_passant() {
        return board.piece_on(source_square).unwrap() == Piece::Pawn
            && (source_square.get_file().to_index() as i8
                - en_passant_square.get_file().to_index() as i8)
                .abs()
                == 1;
    }

    return false;
}

fn is_move_a_castle(board: &Board, chess_move: &ChessMove) -> bool {
    let target_square = chess_move.get_dest();
    let source_square = chess_move.get_source();
    return board.piece_on(source_square).unwrap() == Piece::King
        && (source_square.get_file().to_index() as i8 - target_square.get_file().to_index() as i8)
            .abs()
            > 1;
}

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
    let game = use_state(|| Game::new());
    let move_ply = use_state(|| 0);
    let selected = use_state(|| None);
    let target = use_state(|| None);
    let in_opening_book = use_state(|| true);
    let board = game.current_position();
    let mut board_copy: Board = board.clone();
    let set_selected = {
        let selected = selected.clone();
        Callback::from(move |new_selected| selected.set(new_selected))
    };
    let set_target = {
        let target = target.clone();
        Callback::from(move |new_target| target.set(new_target))
    };
    let reset_game = {
        let game = game.clone();
        Callback::from(move |new_game| game.set(new_game))
    };
    if (*target).is_some() && (*selected).is_some() {
        let mut new_move = ChessMove::new(selected.unwrap(), target.unwrap(), None);
        if target.unwrap().get_rank() == Rank::Eighth
            && board.piece_on(selected.unwrap()).unwrap() == Piece::Pawn
        {
            // then we are promoting a pawn. lets just auto queen for now
            new_move = ChessMove::new(selected.unwrap(), target.unwrap(), Some(Piece::Queen));
        }
        play_move_sound(&board_copy, &new_move, false);
        board.make_move(new_move, &mut board_copy);

        game.set(Game::new_with_board(board_copy));
        move_ply.set(*move_ply + 1);

        // unset the move and selection
        selected.set(None);
        target.set(None);
    } else if board.side_to_move() == Color::Black && game.result().is_none() {
        let timeout = Timeout::new(0, move || {
            if *in_opening_book {
                let ai_move = opening_book_move(board.get_hash());
                if ai_move.is_some() {
                    let ai_move = ai_move.unwrap();
                    play_move_sound(&board_copy, &ai_move, true);
                    board.make_move(ai_move, &mut board_copy);
                } else {
                    // we just got out of opening book, so choose a move on our own now
                    in_opening_book.set(false);
                    let ai_move = choose_move(&board, *move_ply);
                    if ai_move.is_some() {
                        let ai_move = ai_move.unwrap();
                        play_move_sound(&board_copy, &ai_move, true);
                        board.make_move(ai_move, &mut board_copy);
                    }
                }
            } else {
                let ai_move = choose_move(&board, *move_ply);
                if ai_move.is_some() {
                    let ai_move = ai_move.unwrap();
                    play_move_sound(&board_copy, &ai_move, true);
                    board.make_move(ai_move, &mut board_copy);
                }
            }
            game.set(Game::new_with_board(board_copy));
            move_ply.set(*move_ply + 1);
        });
        timeout.forget();
    }

    let board_vec = parse_board(&board_copy);

    // add move circles
    let mut moves = HashSet::new();
    if selected.is_some() {
        let square: Square = selected.unwrap();
        for legal_move in MoveGen::new_legal(&board_copy) {
            if legal_move.get_source() == square {
                moves.insert(legal_move.get_dest());
            }
        }
    }
    // for checking if game has ended
    let game_after_move = Game::new_with_board(board_copy);

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
        {html! {
            if let Some(result) = game_after_move.result() {
                <GameOverScreen result={result} reset_game={reset_game}/>
            }
        }}
        </div>
    }
}
