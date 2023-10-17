use crate::{constants::DEPTH, evaluation::board_eval};
use chess::{Board, CacheTable, ChessMove, MoveGen, EMPTY};
use gloo_console::log;

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
    if depth == 0 {
        let hash = board.get_hash();
        if let Some(evaluation) = transposition_table.get(hash) {
            return (evaluation, None);
        } else {
            let evaluation = board_eval(board, !maximizing_player, move_ply);
            transposition_table.add(board.get_hash(), evaluation);
            return (evaluation, None);
        }
    }
    let mut moves = MoveGen::new_legal(board);
    if maximizing_player {
        let mut best_val = f32::NEG_INFINITY;
        let mut best_move = None;
        let captures = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*captures);
        let mut has_broken = false;
        for legal_move in &mut moves {
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
                    alpha,
                    beta,
                    move_ply + 1,
                )
                .0
            };
            transposition_table.add(board_with_move.get_hash(), evaluation);

            if evaluation > best_val {
                best_val = evaluation;
                best_move = Some(legal_move);
            }
            alpha = f32::max(alpha, evaluation);
            //  if our alpha is >= beta, no need to search any further. PRUNE!
            if alpha >= beta {
                has_broken = true;
                break;
            }
        }
        moves.set_iterator_mask(!EMPTY);
        if !has_broken {
            for legal_move in &mut moves {
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
                        alpha,
                        beta,
                        move_ply + 1,
                    )
                    .0
                };
                transposition_table.add(board_with_move.get_hash(), evaluation);

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
        }
        return (best_val, best_move);
    } else {
        let mut best_val = f32::INFINITY;
        let mut best_move = None;
        let captures = board.color_combined(!board.side_to_move());
        moves.set_iterator_mask(*captures);
        let mut has_broken = false;
        for legal_move in &mut moves {
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
                    alpha,
                    beta,
                    move_ply + 1,
                )
                .0
            };
            transposition_table.add(board_with_move.get_hash(), evaluation);

            if evaluation < best_val {
                best_val = evaluation;
                best_move = Some(legal_move);
            }
            beta = f32::min(beta, evaluation);
            //  if our alpha is >= beta, no need to search any further. PRUNE!
            if alpha >= beta {
                has_broken = true;
                break;
            }
        }
        moves.set_iterator_mask(!EMPTY);
        if !has_broken {
            for legal_move in &mut moves {
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
                        alpha,
                        beta,
                        move_ply + 1,
                    )
                    .0
                };
                transposition_table.add(board_with_move.get_hash(), evaluation);

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
        }
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
    log!(eval);
    if ai_move.is_none() {
        return MoveGen::new_legal(board).next();
    }
    ai_move
}
