/// Represents the file - column on a chessboard
///
/// Possible values are `One` to `Eight`
#[derive(Clone, Copy)]
pub enum File {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}

impl From<File> for u8 {
    fn from(value: File) -> Self {
        value as Self
    }
}
