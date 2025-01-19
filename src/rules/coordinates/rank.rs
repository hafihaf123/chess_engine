/// Represents the rank - row on a chessboard
///
/// Possible values are `A` to `H`
#[derive(Clone, Copy)]
pub enum Rank {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl From<Rank> for u8 {
    fn from(value: Rank) -> Self {
        value as Self
    }
}
