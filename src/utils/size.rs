#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Size {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
    XXXXLarge,
}

impl Size {
    pub fn as_class(&self) -> &'static str {
        match self {
            Size::XSmall => "pf-m-xs",
            Size::Small => "pf-m-sm",
            Size::Medium => "pf-m-md",
            Size::Large => "pf-m-lg",
            Size::XLarge => "pf-m-xl",
            Size::XXLarge => "pf-m-2xl",
            Size::XXXLarge => "pf-m-3xl",
            Size::XXXXLarge => "pf-m-4xl",
        }
    }
}
