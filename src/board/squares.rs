use crate::board::square::colors::{Black, White};
use crate::board::square::ColorBitBoard;

pub struct Squares {
    white: ColorBitBoard<White>,
    black: ColorBitBoard<Black>,
}
