use crate::{
    constants::{CHECKMATE_EVAL, MAX_DEPTH, MAX_EXTENSIONS},
    evaluation::board_eval,
    move_orderer::MoveOrderer,
    transposition_table::{TranspositionTable, Type},
};
use chess::{Board, BoardStatus, ChessMove, Color, MoveGen, Square};
use gloo_console::log;

fn search(
    board: &Board,
    transposition_table: &mut TranspositionTable,
    move_orderer: &mut MoveOrderer,
    ply_remaining: u8,
    ply_searched: u8,
    num_extensions: u8,
    mut alpha: i32,
    mut beta: i32,
    move_ply: u32,
) -> (i32, Option<ChessMove>) {
    let orig_alpha = alpha;
    let maximizing_player = board.side_to_move() == Color::White;
    /* base cases for search function */
    /* 1. we have already seen this position before */
    if let Some(evaluation_move_pair) =
        transposition_table.get(board.get_hash(), ply_remaining, ply_searched, alpha, beta)
    {
        return evaluation_move_pair;
    }

    /* 2. Either we have reached 0 depth or our game finished */
    if ply_remaining <= 0 || board.status() != BoardStatus::Ongoing {
        // if on 0 depth but game is ongoing, then do quinescent search
        let evaluation = if board.status() == BoardStatus::Checkmate && maximizing_player {
            -CHECKMATE_EVAL + ply_searched as i32
        } else if board.status() == BoardStatus::Checkmate {
            CHECKMATE_EVAL - ply_searched as i32
        } else if board.status() == BoardStatus::Stalemate {
            0
        } else {
            quiescence_search(board, alpha, beta, move_orderer, ply_searched)
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
    /* Order moves first by looking at checks, then captures, then the remaining moves */
    let moves: Vec<ChessMove> =
        move_orderer.order_moves(MoveGen::new_legal(board).collect(), board, ply_searched);

    let mut best_val = if maximizing_player {
        /* If we are the maximzing player (i.e. white), we want to get the move with the maximum evaluation,
        so start with the minimum evaluation */
        -CHECKMATE_EVAL
    } else {
        /* If we are the minimizing player (i.e. black), we want to get the move with the minimum evaluation,
        so start with the maximum evaluation */
        CHECKMATE_EVAL
    };
    let mut best_move = None;

    for (i, legal_move) in moves.iter().enumerate() {
        let board_with_move = board.make_move_new(*legal_move);
        let mut curr_extension: u8 = 0;
        // search extensions extend the search whenever our move checked the opponent's king (we want to
        // look deeper into check moves since there are less possible responses by opponent so we can afford to go deeper)
        if board_with_move.checkers().popcnt() > 0 && num_extensions < MAX_EXTENSIONS {
            curr_extension = 1;
        }
        // if i >= 4, then we are towards the middle of our ordered-move list. as a result, we have said that these moves are
        // less likely to be good moves since they were ordered less, so reduce the search depth for these branches
        let search_minimization = if i >= 3 && ply_remaining > 1 { 1 } else { 0 };

        let evaluation = search(
            &board_with_move,
            transposition_table,
            move_orderer,
            ply_remaining - 1 + curr_extension - search_minimization,
            ply_searched + 1,
            num_extensions + curr_extension,
            alpha,
            beta,
            move_ply + 1,
        )
        .0;

        if maximizing_player {
            if evaluation > best_val {
                best_val = evaluation;
                best_move = Some(*legal_move);
            }
            alpha = i32::max(alpha, evaluation);
        } else {
            if evaluation < best_val {
                best_val = evaluation;
                best_move = Some(*legal_move);
            }
            beta = i32::min(beta, evaluation);
        }

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
            // since we have an alpha beta cutoff, this could be a killer move if it isn't a capture
            let is_capture_move = board
                .piece_on(Square::make_square(
                    legal_move.get_dest().get_rank(),
                    legal_move.get_dest().get_file(),
                ))
                .is_some();

            if !is_capture_move {
                move_orderer.add_killer_move(*legal_move, ply_searched);
                // if there is a cutoff earlier in the search tree, then the ply_remaining will be greater
                // also, an earlier cutoff means an obviously bad move such as an opponent blundering a queen. because of this,
                // prioritize early cutoffs by squaring the ply_remaining so that history score is weighted more heavily in
                // favor of early cutoffs than late cutoffs, because a late cutoff could technically be not 100% accurate due to finite
                // search depth.
                let history_score = ply_remaining * ply_remaining;
                move_orderer.add_history(*legal_move, maximizing_player, history_score)
            }

            return (best_val, best_move);
        }
    }
    let entry_type = if best_val < orig_alpha {
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
}

fn quiescence_search(
    board: &Board,
    mut alpha: i32,
    beta: i32,
    move_orderer: &mut MoveOrderer,
    ply_searched: u8,
) -> i32 {
    let evaluation = board_eval(board, 1_______________________1);
    return evaluation;
    if evaluation >= beta {
        return beta; // cutoff - opposing player will not go down this path
    }
    if evaluation > alpha {
        alpha = evaluation;
    }
    let mut moves_iter = MoveGen::new_legal(board);
    let targets = board.color_combined(!board.side_to_move());
    moves_iter.set_iterator_mask(*targets);
    let moves: Vec<ChessMove> = move_orderer.order_moves(moves_iter.collect(), board, ply_searched);
    for capture_move in moves {
        let board_with_capture_move = board.make_move_new(capture_move);
        let evaluation = quiescence_search(
            &board_with_capture_move,
            -beta,
            -alpha,
            move_orderer,
            ply_searched + 1,
        );
        if evaluation >= beta {
            return beta; // cutoff - opposing player will not go down this path
        }
        if alpha > evaluation {
            alpha = evaluation;
        }
    }
    return alpha;
}

pub fn choose_move(board: &Board, move_ply: u32) -> Option<ChessMove> {
    let mut eval = 0;
    let mut ai_move = None;
    let mut transposition_table = TranspositionTable::new();
    let mut move_orderer = MoveOrderer::new();
    for depth in 1..(MAX_DEPTH + 1) {
        (eval, ai_move) = search(
            board,
            &mut transposition_table,
            &mut move_orderer,
            depth,
            0,
            0,
            -CHECKMATE_EVAL,
            CHECKMATE_EVAL,
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
