use chess::Piece;

pub const DEPTH: u16 = 7;

pub const PIECES: &'static [Piece] = &[
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

pub const ENDGAME_PLY: u32 = 50;

pub const CASTLED_BONUS: i16 = 100;
