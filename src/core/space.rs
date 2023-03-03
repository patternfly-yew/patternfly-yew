macro_rules! spacer {
    ($name:ident, $prefix:literal) => {
        #[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
        pub enum $name {
            None,
            XSmall,
            Small,
            Medium,
            Large,
            XLarge,
            XXLarge,
        }

        impl $crate::core::AsClasses for $name {
            fn extend(&self, classes: &mut yew::prelude::Classes) {
                let name = match self {
                    Self::None => concat!($prefix, "-none"),
                    Self::XSmall => concat!($prefix, "-xs"),
                    Self::Small => concat!($prefix, "-sm"),
                    Self::Medium => concat!($prefix, "-md"),
                    Self::Large => concat!($prefix, "-lg"),
                    Self::XLarge => concat!($prefix, "-xl"),
                    Self::XXLarge => concat!($prefix, "-2xl"),
                };
                classes.push(name);
            }
        }
    };
}

spacer!(Spacer, "pf-m-spacer");
spacer!(SpaceItems, "pf-m-space-items");
