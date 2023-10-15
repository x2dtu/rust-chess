use rand::prelude::*;

use std::str::FromStr;

use chess::{ChessMove, Square};
use rand_distr::WeightedIndex;

struct WeightedChessMove {
    chess_move: ChessMove,
    weight: u32,
}

impl WeightedChessMove {
    fn new(chess_move: ChessMove, weight: u32) -> Self {
        Self { chess_move, weight }
    }
}

fn get_weighted_move(moves: &Vec<WeightedChessMove>) -> Option<ChessMove> {
    if moves.len() == 0 {
        return None;
    }
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(moves.iter().map(|move_entry| move_entry.weight)).unwrap();
    Some(moves[dist.sample(&mut rng)].chess_move)
}

fn parse_moves(lines: &mut std::str::Lines<'_>) -> Vec<WeightedChessMove> {
    let mut moves = Vec::new();

    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            break;
        }

        if parts[0] != "pos" {
            let pairs: Vec<String> = parts[0]
                .chars()
                .collect::<Vec<char>>()
                .chunks(2)
                .map(|chunk| chunk.iter().collect())
                .collect();
            let source_square = Square::from_str(&pairs[0]).unwrap();
            let target_square = Square::from_str(&pairs[1]).unwrap();
            let chess_move = ChessMove::new(source_square, target_square, None);
            let weight = parts[1].parse::<u32>().unwrap();
            moves.push(WeightedChessMove::new(chess_move, weight));
        } else {
            break;
        }
    }

    moves
}

pub fn opening_book_move(target: u64) -> Option<ChessMove> {
    let file = std::include_str!("../Book.txt");
    let mut lines = file.lines();

    while let Some(line) = lines.next() {
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            continue;
        }

        if parts[0] == "pos" {
            let current_target = parts[1].parse::<u64>().unwrap();
            if current_target == target {
                return get_weighted_move(&parse_moves(&mut lines));
            }
        }
    }
    None
}
