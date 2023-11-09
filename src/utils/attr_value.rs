use implicit_clone::unsync::IString;
use std::borrow::Cow;

/// Turn an `IString` (or `AttrValue`) into a `Cow<'static, str>`
pub fn attr_value_to_static_cow(value: &IString) -> Cow<'static, str> {
    match value {
        IString::Static(s) => (*s).into(),
        IString::Rc(s) => (*s).to_string().into(),
    }
}
