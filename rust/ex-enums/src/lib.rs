#[derive(Debug, Clone, PartialEq)]
pub enum Numbers {
    Zero,
    SmallNumber(u8),
    BiggerNumber(u32),
    EvenBiggerNumber(u64),
}
