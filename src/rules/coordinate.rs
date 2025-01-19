use crate::rules::coordinates::{File, Rank};

/// Represents the coordinate on a chessboard, consisting of a [`Rank`] and a [`File`]
#[derive(Copy, Clone)]
pub struct Coordinate {
    rank: Rank,
    file: File,
}

impl Coordinate {
    #[must_use]
    pub fn new(rank: Rank, file: File) -> Self {
        Self { rank, file }
    }

    #[must_use]
    pub fn rank_index(&self) -> u8 {
        self.rank.into()
    }

    #[must_use]
    pub fn file_index(&self) -> u8 {
        self.file.into()
    }

    /// Converts the coordinate into a `u8` bit index representation
    ///
    /// # Examples
    ///
    /// ```
    /// # use chess_engine::rules::Coordinate;
    /// # use chess_engine::rules::coordinates::{File, Rank};
    /// let coordinate = Coordinate::new(Rank::F, File::One);
    /// assert_eq!(coordinate.bit_index(), 5);
    /// let coordinate = Coordinate::new(Rank::B, File::Three);
    /// assert_eq!(coordinate.bit_index(), 17);
    /// ```
    #[must_use]
    pub fn bit_index(&self) -> u8 {
        self.rank_index() + 8 * self.file_index()
    }
}
