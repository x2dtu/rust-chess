use crate::{
    constants::{CHECKMATE_EVAL, MAX_DEPTH},
    evaluation::board_eval,
    transposition_table::{TranspositionTable, Type},
};
use chess::{BitBoard, Board, BoardStatus, ChessMove, MoveGen};
use gloo_console::log;

fn score(chess_move: ChessMove, board: &Board) -> i32 {
    let board_with_move = board.make_move_new(chess_move);
    let history_boost = 0;
    if board_with_move.checkers().popcnt() > 0 {
        // then this move is a checking move
        return 300 + history_boost;
    }
    let dest_square = chess_move.get_dest();
    if (BitBoard::set(dest_square.get_rank(), dest_square.get_file())
        & board.color_combined(!board.side_to_move()))
    .popcnt()
        > 0
    {
        // then this move is a capture move
        return 150 + history_boost;
    }
    return history_boost;
}

fn order_moves(moves: Vec<ChessMove>, board: &Board) -> Vec<ChessMove> {
    let mut scored_moves: Vec<(ChessMove, i32)> =
        moves.iter().map(|&m| (m, score(m, board))).collect();

    // sort based on the precomputed scores
    scored_moves.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // extract the sorted moves
    return scored_moves.into_iter().map(|(m, _)| m).collect();
}

fn search(
    board: &Board,
    transposition_table: &mut TranspositionTable,
    ply_remaining: u8,
    ply_searched: u8,
    maximizing_player: bool,
    alpha_p: i32,
    beta_p: i32,
    move_ply: u32,
) -> (i32, Option<ChessMove>) {
    let mut alpha = alpha_p;
    let mut beta = beta_p;

    /* base cases for search function */
    /* 1. we have already seen this position before */
    if let Some(evaluation_move_pair) =
        transposition_table.get(board.get_hash(), ply_remaining, ply_searched, alpha, beta)
    {
        return evaluation_move_pair;
    }

    /* 2. Either we have reached 0 depth or our game finished */
    if ply_remaining == 0 || board.status() != BoardStatus::Ongoing {
        // if on 0 depth but game is ongoing, then do quinescent search
        let evaluation = if board.status() == BoardStatus::Checkmate && maximizing_player {
            -CHECKMATE_EVAL
        } else if board.status() == BoardStatus::Checkmate {
            CHECKMATE_EVAL
        } else if board.status() == BoardStatus::Stalemate {
            0
        } else {
            board_eval(board, !maximizing_player, move_ply)
        };
        transposition_table.add(
            board.get_hash(),
            evaluation,
            ply_remaining,
            Type::Exact,
            None,
            ply_searched,
        );
        return (evaluation, None);
    }
    /* Generate all the legal moves and iterate over them */
    let moves: Vec<ChessMove> = order_moves(MoveGen::new_legal(board).collect(), board);

    if maximizing_player {
        /* If we are the maximzing player (i.e. white), get the move with the maximum evaluation */
        let mut best_val = -CHECKMATE_EVAL;
        let mut best_move = None;
        /* Order moves first by looking at checks, then captures, then the remaining moves */
        for legal_move in moves {
            let board_with_move = board.make_move_new(legal_move);
            let evaluation = search(
                &board_with_move,
                transposition_table,
                ply_remaining - 1,
                ply_searched + 1,
                !maximizing_player,
                alpha,
                beta,
                move_ply + 1,
            )
            .0;

            if evaluation > best_val {
                best_val = evaluation;
                best_move = Some(legal_move);
            }
            alpha = i32::max(alpha, evaluation);
            //  if our alpha is >= beta, no need to search any further. PRUNE!
            if alpha >= beta {
                transposition_table.add(
                    board_with_move.get_hash(),
                    evaluation,
                    ply_remaining,
                    Type::LowerBound,
                    best_move,
                    ply_searched,
                );
                return (best_val, best_move);
            }
        }
        let entry_type = if best_val < alpha_p {
            Type::UpperBound
        } else {
            Type::Exact
        };
        transposition_table.add(
            board.get_hash(),
            best_val,
            ply_remaining,
            entry_type,
            best_move,
            ply_searched,
        );
        return (best_val, best_move);
    } else {
        /* If we are the minimizing player (i.e. black), get the move with the minimum evaluation */
        let mut best_val = CHECKMATE_EVAL;
        let mut best_move: Option<ChessMove> = None;
        for legal_move in moves {
            let board_with_move = board.make_move_new(legal_move);
            let evaluation = search(
                &board_with_move,
                transposition_table,
                ply_remaining - 1,
                ply_searched + 1,
                !maximizing_player,
                alpha,
                beta,
                move_ply + 1,
            )
            .0;

            if evaluation < best_val {
                best_val = evaluation;
                best_move = Some(legal_move);
            }
            beta = i32::min(beta, evaluation);
            //  if our alpha is >= beta, no need to search any further. PRUNE!
            if alpha >= beta {
                transposition_table.add(
                    board_with_move.get_hash(),
                    evaluation,
                    ply_remaining,
                    Type::LowerBound,
                    best_move,
                    ply_searched,
                );
                return (best_val, best_move);
            }
        }
        let entry_type = if best_val < alpha_p {
            Type::UpperBound
        } else {
            Type::Exact
        };
        transposition_table.add(
            board.get_hash(),
            best_val,
            ply_remaining,
            entry_type,
            best_move,
            ply_searched,
        );
        // log!(
        //     best_val,
        //     if best_move.is_some() {
        //         best_move.unwrap().to_string()
        //     } else {
        //         "none".to_owned()
        //     }
        // );
        return (best_val, best_move);
    }
}

pub fn choose_move(board: &Board, move_ply: u32, is_white: bool) -> Option<ChessMove> {
    let mut eval = 0;
    let mut ai_move = None;
    let mut transposition_table = TranspositionTable::new();
    for depth in 1..(MAX_DEPTH + 1) {
        (eval, ai_move) = search(
            board,
            &mut transposition_table,
            depth,
            0,
            is_white,
            -CHECKMATE_EVAL,
            CHECKMATE_EVAL,
            move_ply,
        );
    }
    // let mut transposition_table = TranspositionTable::new();
    // let (eval, ai_move) = search(
    //     board,
    //     &mut transposition_table,
    //     MAX_DEPTH,
    //     0,
    //     is_white,
    //     -CHECKMATE_EVAL,
    //     CHECKMATE_EVAL,
    //     move_ply,
    // );

    log!(eval);
    if ai_move.is_none() {
        log!("I can't find a good move to save me...");
        return MoveGen::new_legal(board).next();
    }
    ai_move
}
