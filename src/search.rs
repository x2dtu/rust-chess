use crate::{constants::DEPTH, evaluation::board_eval};
use chess::{BitBoard, Board, BoardStatus, CacheTable, ChessMove, MoveGen};
use gloo_console::log;

fn score(
    chess_move: ChessMove,
    board: &Board,
    iterative_deepening_ordering_table: &CacheTable<f32>,
) -> f32 {
    let mut board_with_move = board.clone();
    board.make_move(chess_move, &mut board_with_move);
    let history_boost = iterative_deepening_ordering_table
        .get(board_with_move.get_hash())
        .unwrap_or(0.0);
    if board_with_move.checkers().popcnt() > 0 {
        // then this move is a checking move
        return 300.0 + history_boost;
    }
    let dest_square = chess_move.get_dest();
    if (BitBoard::set(dest_square.get_rank(), dest_square.get_file())
        & board.color_combined(!board.side_to_move()))
    .popcnt()
        > 0
    {
        // then this move is a capture move
        return 150.0 + history_boost;
    }
    return history_boost;
}

fn order_moves(
    moves: Vec<ChessMove>,
    board: &Board,
    iterative_deepening_ordering_table: &CacheTable<f32>,
) -> Vec<ChessMove> {
    let mut scored_moves: Vec<(ChessMove, f32)> = moves
        .iter()
        .map(|&m| (m, score(m, board, iterative_deepening_ordering_table)))
        .collect();

    // sort based on the precomputed scores
    scored_moves.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // extract the sorted moves
    return scored_moves.into_iter().map(|(m, _)| m).collect();
}

fn search(
    board: &Board,
    depth: u16,
    transposition_table: &mut CacheTable<f32>,
    iterative_deepening_ordering_table: &mut CacheTable<f32>,
    maximizing_player: bool,
    alpha_p: f32,
    beta_p: f32,
    move_ply: u32,
) -> (f32, Option<ChessMove>) {
    let mut alpha = alpha_p;
    let mut beta = beta_p;
    /* base case for search function */
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        let hash = board.get_hash();
        if let Some(evaluation) = transposition_table.get(hash) {
            return (evaluation, None);
        } else {
            let evaluation = if board.status() == BoardStatus::Checkmate && maximizing_player {
                f32::NEG_INFINITY
            } else if board.status() == BoardStatus::Checkmate {
                f32::INFINITY
            } else if board.status() == BoardStatus::Stalemate {
                0.0
            } else {
                board_eval(board, !maximizing_player, move_ply)
            };
            transposition_table.add(board.get_hash(), evaluation);
            // iterative_deepening_ordering_table.add(board.get_hash(), evaluation);
            return (evaluation, None);
        }
    }
    /* Generate all the legal moves and iterate over them */
    let moves: Vec<ChessMove> = order_moves(
        MoveGen::new_legal(board).collect(),
        board,
        iterative_deepening_ordering_table,
    );

    if maximizing_player {
        /* If we are the maximzing player (i.e. white), get the move with the maximum evaluation */
        let mut best_val = f32::NEG_INFINITY;
        let mut best_move = None;
        /* Order moves first by looking at checks, then captures, then the remaining moves */
        for legal_move in moves {
            let mut board_with_move = board.clone();
            board.make_move(legal_move, &mut board_with_move);
            let evaluation_option = transposition_table.get(board_with_move.get_hash());
            let evaluation = if evaluation_option.is_some() {
                evaluation_option.unwrap()
            } else {
                search(
                    &board_with_move,
                    depth - 1,
                    transposition_table,
                    iterative_deepening_ordering_table,
                    !maximizing_player,
                    alpha,
                    beta,
                    move_ply + 1,
                )
                .0
            };
            transposition_table.add(board_with_move.get_hash(), evaluation);
            // iterative_deepening_ordering_table.add(board_with_move.get_hash(), evaluation);

            if evaluation > best_val {
                best_val = evaluation;
                best_move = Some(legal_move);
            }
            alpha = f32::max(alpha, evaluation);
            //  if our alpha is >= beta, no need to search any further. PRUNE!
            if alpha >= beta {
                break;
            }
        }
        return (best_val, best_move);
    } else {
        /* If we are the minimizing player (i.e. black), get the move with the minimum evaluation */
        let mut best_val = f32::INFINITY;
        let mut best_move: Option<ChessMove> = None;
        for legal_move in moves {
            let mut board_with_move = board.clone();
            board.make_move(legal_move, &mut board_with_move);
            let evaluation_option = transposition_table.get(board_with_move.get_hash());
            let evaluation = if evaluation_option.is_some() {
                evaluation_option.unwrap()
            } else {
                search(
                    &board_with_move,
                    depth - 1,
                    transposition_table,
                    iterative_deepening_ordering_table,
                    !maximizing_player,
                    alpha,
                    beta,
                    move_ply + 1,
                )
                .0
            };
            transposition_table.add(board_with_move.get_hash(), evaluation);
            // iterative_deepening_ordering_table.add(board_with_move.get_hash(), evaluation);

            if evaluation < best_val {
                best_val = evaluation;
                best_move = Some(legal_move);
            }
            beta = f32::min(beta, evaluation);
            //  if our alpha is >= beta, no need to search any further. PRUNE!
            if alpha >= beta {
                break;
            }
        }
        return (best_val, best_move);
    }
}

pub fn choose_move(board: &Board, move_ply: u32) -> Option<ChessMove> {
    let mut iterative_deepening_ordering_table = CacheTable::new(65536, 0.0);
    let mut eval = 0.0;
    let mut ai_move = None;
    for depth in DEPTH..(DEPTH + 1) {
        let mut transposition_table = CacheTable::new(65536, 0.0);
        (eval, ai_move) = search(
            board,
            depth,
            &mut transposition_table,
            &mut iterative_deepening_ordering_table,
            false,
            f32::NEG_INFINITY,
            f32::INFINITY,
            move_ply,
        );
    }

    log!(eval);
    if ai_move.is_none() {
        log!("I can't find a good move to save me...");
        return MoveGen::new_legal(board).next();
    }
    ai_move
}
