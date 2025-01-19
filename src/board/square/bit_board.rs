use crate::board::square::bitmap::Bitmap;
use crate::board::square::pieces::PieceType;
use crate::rules::{Coordinate, PieceMove};

//   / - - - - - - - - \
// 8 | 0 0 0 0 0 0 0 0 |
// 7 | 0 0 0 0 0 0 0 0 |
// 6 | 0 0 0 0 0 0 0 0 |
// 5 | 0 0 0 0 0 0 0 0 |
// 4 | 0 0 0 0 0 0 0 0 |
// 3 | 0 0 0 1 0 0 0 0 |
// 2 | 0 0 0 0 0 0 0 0 |
// 1 | 0 0 0 0 0 0 0 0 |
//   \ - - - - - - - - /
//     A B C D E F G H
pub struct BitBoard<T: PieceType> {
    bitmap: Bitmap,
    piece_type: std::marker::PhantomData<T>,
}

impl<T: PieceType> BitBoard<T> {
    #[must_use]
    pub fn new(bitmap: u64) -> Self {
        Self {
            bitmap: bitmap.into(),
            piece_type: std::marker::PhantomData,
        }
    }

    #[must_use]
    pub fn bitmap(&self) -> Bitmap {
        self.bitmap
    }

    #[must_use]
    pub fn contains_piece(&self, coordinate: Coordinate) -> bool {
        self.bitmap().contains(coordinate.bit_index())
    }

    pub fn make_a_move(&mut self, _piece_move: &PieceMove<T>) {
        todo!()
    }
}
