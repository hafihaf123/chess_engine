use crate::board::castling::Status;

pub struct Rights {
    white_queen_side: Status,
    white_king_side: Status,
    black_queen_side: Status,
    black_king_side: Status,
}
