use crate::board::castling;
use crate::board::square::Color;

pub struct GameState {
    active_color: Color,
    castling_rights: castling::Rights,
    full_move_number: u32,
}
