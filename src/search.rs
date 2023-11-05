use crate::{constants::DEPTH, evaluation::board_eval};
use chess::{BitBoard, Board, BoardStatus, CacheTable, ChessMove, MoveGen, EMPTY};
use gloo_console::log;

/**
 * This method will get a bit board of all the legal moves from param board that put opponent into check
 */
fn get_checking_moves(board: &Board) -> BitBoard {
    let mut checking_bit_board = BitBoard::new(0);

    let moves = MoveGen::new_legal(board);
    for legal_move in moves {
        let mut board_with_move = board.clone();
        board.make_move(legal_move, &mut board_with_move);
        if board_with_move.checkers().popcnt() > 0 {
            let dest_square = legal_move.get_dest();
            checking_bit_board |= BitBoard::set(dest_square.get_rank(), dest_square.get_file());
        }
    }
    return checking_bit_board;
}

fn search(
    board: &Board,
    depth: u16,
    transposition_table: &mut CacheTable<f32>,
    maximizing_player: bool,
    alpha_p: f32,
    beta_p: f32,
    move_ply: u32,
) -> (f32, Option<ChessMove>) {
    let mut alpha = alpha_p;
    let mut beta = beta_p;
    /* define a helper closure that will help us with iterative over our legal moves */
    let mut iterate_over_moves = |moves: &mut MoveGen,
                                  alpha: &mut f32,
                                  beta: &mut f32,
                                  best_val: &mut f32,
                                  best_move: &mut Option<ChessMove>,
                                  has_broken: &mut bool| {
        if !*has_broken {
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
                        !maximizing_player,
                        *alpha,
                        *beta,
                        move_ply + 1,
                    )
                    .0
                };
                transposition_table.add(board_with_move.get_hash(), evaluation);

                if maximizing_player {
                    if evaluation > *best_val {
                        *best_val = evaluation;
                        *best_move = Some(legal_move);
                    }
                    *alpha = f32::max(*alpha, evaluation);
                } else {
                    if evaluation < *best_val {
                        *best_val = evaluation;
                        *best_move = Some(legal_move);
                    }
                    *beta = f32::min(*beta, evaluation);
                }
                //  if our alpha is >= beta, no need to search any further. PRUNE!
                if *alpha >= *beta {
                    *has_broken = true;
                    break;
                }
            }
        }
    };

    /* base case for search function */
    if depth == 0 || board.status() != BoardStatus::Ongoing {
        let hash = board.get_hash();
        if let Some(evaluation) = transposition_table.get(hash) {
            return (evaluation, None);
        } else {
            if board.status() == BoardStatus::Checkmate {
                return if maximizing_player {
                    (f32::NEG_INFINITY, None)
                } else {
                    (f32::INFINITY, None)
                };
            }
            let evaluation = board_eval(board, !maximizing_player, move_ply);
            transposition_table.add(board.get_hash(), evaluation);
            return (evaluation, None);
        }
    }
    /* Generate all the legal moves and iterate over them */
    let mut moves = MoveGen::new_legal(board);
    if maximizing_player {
        /* If we are the maximzing player (i.e. white), get the move with the maximum evaluation */
        let mut best_val = f32::NEG_INFINITY;
        let mut best_move = None;
        let mut has_broken = false;
        /* Order moves first by looking at checks, then captures, then the remaining moves */
        // let checks = get_checking_moves(board);
        // moves.set_iterator_mask(checks);
        // iterate_over_moves(
        //     &mut moves,
        //     &mut alpha,
        //     &mut beta,
        //     &mut best_val,
        //     &mut best_move,
        //     &mut has_broken,
        // );
        let captures = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*captures);
        iterate_over_moves(
            &mut moves,
            &mut alpha,
            &mut beta,
            &mut best_val,
            &mut best_move,
            &mut has_broken,
        );
        moves.set_iterator_mask(!EMPTY);
        iterate_over_moves(
            &mut moves,
            &mut alpha,
            &mut beta,
            &mut best_val,
            &mut best_move,
            &mut has_broken,
        );
        return (best_val, best_move);
    } else {
        /* If we are the minimizing player (i.e. black), get the move with the minimum evaluation */
        let mut best_val = f32::INFINITY;
        let mut best_move: Option<ChessMove> = None;
        let mut has_broken = false;
        /* Order moves first by looking at checks, then captures, then the remaining moves */
        let checks = get_checking_moves(board);
        moves.set_iterator_mask(checks);
        iterate_over_moves(
            &mut moves,
            &mut alpha,
            &mut beta,
            &mut best_val,
            &mut best_move,
            &mut has_broken,
        );
        let captures = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*captures);
        iterate_over_moves(
            &mut moves,
            &mut alpha,
            &mut beta,
            &mut best_val,
            &mut best_move,
            &mut has_broken,
        );
        moves.set_iterator_mask(!EMPTY);
        iterate_over_moves(
            &mut moves,
            &mut alpha,
            &mut beta,
            &mut best_val,
            &mut best_move,
            &mut has_broken,
        );
        return (best_val, best_move);
    }
}

pub fn choose_move(board: &Board, move_ply: u32) -> Option<ChessMove> {
    let mut transposition_table = CacheTable::new(65536, 0.0);
    let (eval, ai_move) = search(
        board,
        DEPTH,
        &mut transposition_table,
        false,
        f32::NEG_INFINITY,
        f32::INFINITY,
        move_ply,
    );
    log!(eval, "made a move");
    if ai_move.is_none() {
        log!("I can't find a good move to save me...");
        return MoveGen::new_legal(board).next();
    }
    ai_move
}
