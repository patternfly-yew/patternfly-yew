/// Definition for orientations
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Orientation {
    Left,
    Right,
    Top,
    Bottom,
}

impl Orientation {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Orientation::Left => vec!["pf-m-left"],
            Orientation::Right => vec!["pf-m-right"],
            Orientation::Top => vec!["pf-m-top"],
            Orientation::Bottom => vec!["pf-m-bottom"],
        }
    }
}
