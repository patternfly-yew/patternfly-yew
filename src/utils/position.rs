/// Definition for positions
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Position {
    Left,
    Right,
    Top,
}

impl Default for Position {
    fn default() -> Self {
        Self::Left
    }
}
