#[derive(
    Debug,
    Hash,
    Eq,
    PartialEq,
    strum_macros::EnumIter,
    strum_macros::Display,
    Copy,
    Clone,
    Ord,
    PartialOrd,
)]
pub enum Rank {
    ACE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    JACK,
    QUEEN,
    KING,
}
