use chess::Piece;

pub const DEPTH: u16 = 5;

pub const PIECES: &'static [Piece] = &[
    Piece::Pawn,
    Piece::Knight,
    Piece::Bishop,
    Piece::Rook,
    Piece::Queen,
    Piece::King,
];

pub const OPENING_BOOK_FILE_NAME: &str = "Book.txt";
