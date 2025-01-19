use crate::board::square::pieces::PieceType;
use crate::rules::Coordinate;

pub struct PieceMove<T: PieceType> {
    coordinate_from: Coordinate,
    coordinate_to: Coordinate,

    piece_type: std::marker::PhantomData<T>,
}
