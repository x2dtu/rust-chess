use chess::{BitBoard, Board, ChessMove, Color, Square, NUM_SQUARES};

use crate::{constants::MAX_KILLER_MOVE_PLY, evaluation::get_count_of_piece};

const CHECK_BONUS: i32 = 250;

const PROMOTION_BONUS: i32 = 800;

const KILLER_BONUS: i32 = 400;

const GOOD_CAPTURE_BONUS: i32 = 600;

const BAD_CAPTURE_BONUS: i32 = 200;

#[derive(Default, Clone, Copy)]
pub struct KillerMoveEntry {
    // we store up to two killer moves at a given killer move ply. this is because
    // 2 moves strikes a good balance between having a lot of information to help move ordering
    // while also being memory efficient. storing more than 2 moves at each ply gives diminishing returns
    pub first_move: ChessMove,
    pub second_move: ChessMove,
}

impl KillerMoveEntry {
    pub fn add_move(&mut self, new_killer_move: ChessMove) {
        if self.first_move != new_killer_move {
            // then we need to move the current first move into the second move slot
            // if there was a move that was previously stored in the second move slot, then that move will be evicted
            self.second_move = self.first_move;
            self.first_move = new_killer_move;
        }
    }
    pub fn contains_move(&self, chess_move: ChessMove) -> bool {
        self.first_move == chess_move || self.second_move == chess_move
    }
}

pub struct MoveOrderer {
    history: [[[u8; 2]; NUM_SQUARES]; NUM_SQUARES],
    killer_moves: [KillerMoveEntry; MAX_KILLER_MOVE_PLY],
}

impl MoveOrderer {
    pub fn new() -> MoveOrderer {
        MoveOrderer {
            history: [[[0; 2]; NUM_SQUARES]; NUM_SQUARES],
            killer_moves: [KillerMoveEntry::default(); MAX_KILLER_MOVE_PLY],
        }
    }
    pub fn order_moves(
        &self,
        moves: Vec<ChessMove>,
        board: &Board,
        ply_searched: u8,
    ) -> Vec<ChessMove> {
        let mut scored_moves: Vec<(ChessMove, i32)> = moves
            .iter()
            .map(|&m| (m, self.score(m, board, ply_searched)))
            .collect();

        // sort based on the precomputed scores
        scored_moves.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // extract the sorted moves
        return scored_moves.into_iter().map(|(m, _)| m).collect();
    }

    fn score(&self, chess_move: ChessMove, board: &Board, ply_searched: u8) -> i32 {
        let board_with_move = board.make_move_new(chess_move);
        let dest_square = chess_move.get_dest();
        let source_square = chess_move.get_source();

        let mut score: i32 = 0;

        let our_piece = board.piece_on(source_square).unwrap();

        if board_with_move.checkers().popcnt() > 0 {
            // then this move is a checking move
            score += CHECK_BONUS;
        }

        if chess_move.get_promotion().is_some() {
            score += PROMOTION_BONUS;
        }

        let opponent_can_capture = bb_contains(
            board_with_move.color_combined(board_with_move.side_to_move()),
            dest_square,
        );

        /* Figure out if our move is a capture or not */
        let en_passant_square = board.en_passant();
        let piece_at_dest = board.piece_on(dest_square);

        let captured_piece_option = if piece_at_dest.is_some() {
            piece_at_dest
        } else if en_passant_square.is_some()
            && board.piece_on(en_passant_square.unwrap()).is_some()
        {
            board.piece_on(en_passant_square.unwrap())
        } else {
            None
        };

        if let Some(captured_piece) = captured_piece_option {
            // then this move is a capture move

            let captured_piece_value = get_count_of_piece(captured_piece);
            let our_piece_value = get_count_of_piece(our_piece);
            let difference = our_piece_value as i32 - captured_piece_value as i32;
            if opponent_can_capture {
                // then opponent can recapture us
                score += if difference > 0 {
                    GOOD_CAPTURE_BONUS
                } else {
                    BAD_CAPTURE_BONUS
                } + difference;
            } else {
                score += GOOD_CAPTURE_BONUS + difference;
            }

            // let our_piece = score += 200;
        } else {
            let index = ply_searched as usize;
            // if we aren't a capture move or from the quiescence search which was formed from an end-search capture sequence,
            // then we may be a killer move. Killer moves are moves which cause an alpha-beta cutoff
            let is_killer_move =
                index < MAX_KILLER_MOVE_PLY && self.killer_moves[index].contains_move(chess_move);

            if is_killer_move {
                score += KILLER_BONUS;
            }
            let player_dim: usize = if board.side_to_move() == Color::White {
                0
            } else {
                1
            };
            // moves in history which have a greater score will be prioritized. this is a dynamic way to improve move ordering
            // as the course of the game will shape how the history table's elements get stored.
            score += self.history[chess_move.get_source().to_index()]
                [chess_move.get_dest().to_index()][player_dim] as i32;
        }

        if opponent_can_capture {
            score -= 25;
        }

        return score;
    }

    pub fn add_killer_move(&mut self, killer_move: ChessMove, ply_searched: u8) {
        let index = ply_searched as usize;
        if index >= self.killer_moves.len() {
            return;
        }
        if self.killer_moves[index].first_move != killer_move {
            self.killer_moves[index].second_move = self.killer_moves[index].first_move;
            self.killer_moves[index].first_move = killer_move;
        }
    }

    pub fn add_history(&mut self, chess_move: ChessMove, maximizing_player: bool, score: u8) {
        let player_dim: usize = if maximizing_player { 0 } else { 1 };
        self.history[chess_move.get_source().to_index()][chess_move.get_dest().to_index()]
            [player_dim] = score;
    }
}

pub fn bb_contains(bb: &BitBoard, target_square: Square) -> bool {
    (bb & BitBoard::from_square(target_square)).popcnt() > 0
}
