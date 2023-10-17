use crate::constants::{CASTLED_BONUS, ENDGAME_PLY, PIECES};
use chess::{Board, BoardStatus, CastleRights, Color, Piece, Square};

pub fn board_eval(board: &Board, maximizing_player: bool, move_ply: u32) -> f32 {
    let color = if maximizing_player {
        Color::White
    } else {
        Color::Black
    };
    if board.status() == BoardStatus::Checkmate {
        return if color == Color::White {
            f32::NEG_INFINITY
        } else {
            f32::INFINITY
        };
    }
    let mut eval = 0;
    let material_count = count_material(board);
    let king_safety = evaluate_king_safety(board, color, move_ply);
    let castle_status = can_castle(board, color, move_ply);
    eval += material_count + king_safety + castle_status;

    if board.status() == BoardStatus::Stalemate {
        return if eval > 0 {
            f32::NEG_INFINITY
        } else if eval < 0 {
            f32::INFINITY
        } else {
            0.0
        };
    }
    return eval as f32;
}

fn count_material(board: &Board) -> i16 {
    let mut material = 0;
    for piece in PIECES {
        let piece_bb = board.pieces(*piece);
        material += ((piece_bb & board.color_combined(Color::White)).popcnt()
            * get_count_of_piece(*piece)) as i16;
        material -= ((piece_bb & board.color_combined(Color::Black)).popcnt()
            * get_count_of_piece(*piece)) as i16;
    }
    material
}

fn get_count_of_piece(piece: Piece) -> u32 {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 300,
        Piece::Bishop => 320,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 10000,
    }
}

fn evaluate_king_safety(board: &Board, color: Color, move_ply: u32) -> i16 {
    let king_square = board.king_square(color);
    if move_ply >= ENDGAME_PLY {
        return 0; // no longer care about king safety
    }
    fn is_rook_not_on_square(board: &Board, square: Square) -> bool {
        return board.piece_on(square).is_none() || board.piece_on(square).unwrap() != Piece::Rook;
    }

    // if white is castled
    if ((king_square == Square::G1 || king_square == Square::H1)
        && is_rook_not_on_square(board, Square::H1))
        || ((king_square == Square::C1 || king_square == Square::B1 || king_square == Square::A1)
            && is_rook_not_on_square(board, Square::A1)
            && is_rook_not_on_square(board, Square::B1))
    {
        return CASTLED_BONUS;
    }
    // if black is castled
    else if ((king_square == Square::G8 || king_square == Square::H8)
        && is_rook_not_on_square(board, Square::H8))
        || ((king_square == Square::C8 || king_square == Square::B8 || king_square == Square::A8)
            && is_rook_not_on_square(board, Square::A8)
            && is_rook_not_on_square(board, Square::B8))
    {
        return -CASTLED_BONUS;
    }
    return 0;
}

fn can_castle(board: &Board, color: Color, move_ply: u32) -> i16 {
    let color_multiplier = if color == Color::White { 1 } else { -1 };
    if board.castle_rights(color) == CastleRights::NoRights && move_ply < ENDGAME_PLY {
        -200 * color_multiplier
    } else {
        0
    }
}
