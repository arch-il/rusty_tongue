#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WordStatus {
    NotAWord = 0,
    Learning = 1,
    Mastered = 2,
    New = 3,
}
