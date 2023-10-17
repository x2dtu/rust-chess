use chess::Piece;

pub const DEPTH: u16 = 6;

pub const PIECES: &'static [Piece] = &[
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];
