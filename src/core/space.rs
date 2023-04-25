macro_rules! spacer {
    ($(#[$outer:meta])*
    $name:ident, $prefix:literal) => {

        $(#[$outer])*
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
            fn extend_classes(&self, classes: &mut yew::prelude::Classes) {
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

spacer!(
    /// Spacer definition
    Spacer,
    "pf-m-spacer"
);

spacer!(
    /// Spacer definition for items
    SpaceItems,
    "pf-m-space-items"
);
