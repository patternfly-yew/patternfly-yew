use std::ops::Not;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Order {
    Ascending,
    Descending,
}

impl Not for Order {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }
}
