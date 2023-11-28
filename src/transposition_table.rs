use std::cmp::Ordering;

use chess::{CacheTable, ChessMove};

use crate::constants::CHECKMATE_EVAL;

#[derive(Copy, Clone, Default, PartialEq)]
pub enum Type {
    #[default]
    Exact,
    UpperBound,
    LowerBound,
}

#[derive(Copy, Clone, Default)]
struct Entry {
    evaluation: i32,
    best_move: Option<ChessMove>,
    entry_type: Type,
    depth: u8,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.evaluation == other.evaluation
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.evaluation.partial_cmp(&other.evaluation)
    }
}

pub struct TranspositionTable {
    tt: CacheTable<Entry>,
}

impl TranspositionTable {
    pub fn new() -> TranspositionTable {
        TranspositionTable {
            tt: CacheTable::new(65536, Entry::default()),
        }
    }
    pub fn get(
        &self,
        zobrist_hash: u64,
        depth: u8,
        ply_searched: u8,
        alpha: i32,
        beta: i32,
    ) -> Option<(i32, Option<ChessMove>)> {
        if let Some(entry) = self.tt.get(zobrist_hash) {
            if entry.depth < depth {
                return None; // we haven't evaluated this position before at the specified depth
            }
            let corrected_evaluation = get_optimized_mate_score(entry.evaluation, ply_searched);
            if entry.entry_type == Type::Exact
                || (entry.entry_type == Type::UpperBound && corrected_evaluation <= alpha)
                || (entry.entry_type == Type::LowerBound && corrected_evaluation >= beta)
            {
                return Some((corrected_evaluation, entry.best_move));
            }
        }
        return None;
    }
    pub fn add(
        &mut self,
        zobrist_hash: u64,
        evaluation: i32,
        depth: u8,
        entry_type: Type,
        best_move: Option<ChessMove>,
        ply_searched: u8,
    ) {
        self.tt.add(
            zobrist_hash,
            Entry {
                evaluation: store_optimized_mate_score(evaluation, ply_searched),
                best_move,
                depth,
                entry_type,
            },
        );
    }
}

/*
When a mate score is being stored in the transposition table, it's adjusted based on ply_searched.
this adjustment is necessary because the mate score is a relative value. For instance, M3 (mate in 3)
means 3 moves from the current depth of the search. by adding ply_searched to the mate score, we are
essentially storing the absolute number of moves to mate from the root of the search tree. this conversion
is crucial because the depth of the search changes as the ai explores different branches of the game tree
*/
#[inline]
fn store_optimized_mate_score(eval: i32, ply_searched: u8) -> i32 {
    let sign = if eval < 0 { -1 } else { 1 };
    return if is_mate_eval(eval) {
        sign * (sign * eval + ply_searched as i32)
    } else {
        eval
    };
}
/*
Meanwhile, when a mate score is retrieved, it needs to be converted back to a relative value, i.e.,
relative to the current depth of the search. This is done by subtracting ply_searched from the stored score.
this subtraction recalibrates the mate score to be relevant to the current search depth. for instance, if
M5 was stored at a depth of 2, and it's being retrieved at a depth of 4, the recalculated score should
reflect M3. We care about the ply_searched in general because we want to prioritize faster mates than
slower mates i.e. M2 over M6
*/
#[inline]
fn get_optimized_mate_score(eval: i32, ply_searched: u8) -> i32 {
    let sign = if eval < 0 { -1 } else { 1 };
    return if is_mate_eval(eval) {
        sign * (sign * eval - ply_searched as i32)
    } else {
        eval
    };
}

#[inline]
fn is_mate_eval(eval: i32) -> bool {
    eval.abs() > CHECKMATE_EVAL - 100 // extra leeway
}
