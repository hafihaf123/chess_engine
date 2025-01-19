use crate::board::square::colors::ColorType;
use crate::board::square::pieces::r#type::{Bishop, King, Knight, Pawn, Queen};
use crate::board::square::BitBoard;

pub struct ColorBitBoard<T: ColorType> {
    pawns: BitBoard<Pawn>,
    knights: BitBoard<Knight>,
    bishops: BitBoard<Bishop>,
    queens: BitBoard<Queen>,
    kings: BitBoard<King>, // this looks dumb, but let's do it like this for now

    color: std::marker::PhantomData<T>,
}
