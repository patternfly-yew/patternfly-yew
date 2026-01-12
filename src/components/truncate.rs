use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TruncateContent {
    Default(String),
    Middle(String, String),
    Start(String),
}

impl TruncateContent {
    pub fn middle<S: Into<String>, E: Into<String>>(start: S, end: E) -> Self {
        Self::Middle(start.into(), end.into())
    }

    pub fn start<S: Into<String>>(start: S) -> Self {
        Self::Start(start.into())
    }
}

impl From<String> for TruncateContent {
    fn from(value: String) -> Self {
        Self::Default(value)
    }
}

impl From<&str> for TruncateContent {
    fn from(value: &str) -> Self {
        Self::Default(value.to_string())
    }
}

impl IntoPropValue<TruncateContent> for String {
    fn into_prop_value(self) -> TruncateContent {
        TruncateContent::Default(self)
    }
}

impl IntoPropValue<TruncateContent> for &str {
    fn into_prop_value(self) -> TruncateContent {
        TruncateContent::Default(self.to_string())
    }
}

/// Helps creating content for [`Truncate`].
pub trait IntoTruncateContent {
    /// Truncate at the start of the content.
    fn truncate_start(self) -> TruncateContent;

    /// Truncate `num` characters before the end of the string.
    fn truncate_before(self, num: usize) -> TruncateContent;
}

impl<T: ToString> IntoTruncateContent for T {
    fn truncate_start(self) -> TruncateContent {
        TruncateContent::Start(self.to_string())
    }

    /// This function is supposed to truncate `num` characters before the end of the string.
    ///
    /// ## Bytes, Code Points, and Grapheme Clusters
    ///
    /// However, what it actually does is to truncate the string at the next Unicode code point,
    /// after `num` bytes (not characters). This is quick and should work reasonably well with
    /// the Latin 1 character set (or, UTF-8 characters which are represented by a single byte).
    ///
    /// Given a string with multi-byte code points, or even grapheme clusters (user-perceived
    /// characters, which may consists of multiple Unicode code points), this will split at the
    /// wrong location.
    ///
    /// It will still split, and not skip any data. But it might lead to an unexpected (shorter)
    /// end section.
    ///
    /// What about an actual correct implementation? That would be possible by using an additional
    /// dependency. It would also need to count all code points and grapheme clusters from the
    /// start of the string. The question is: is that worth it? Maybe, maybe not!?
    fn truncate_before(self, num: usize) -> TruncateContent {
        let s = self.to_string();
        let len = s.len();

        if num == 0 {
            return TruncateContent::Default(s);
        }

        if num > len {
            return TruncateContent::Start(s);
        }

        let mut end = len - num;
        loop {
            if end == 0 {
                return TruncateContent::Start(s);
            }

            if s.is_char_boundary(end) {
                break;
            }

            // we can't get negative, as we exit the loop when end == 0
            end -= 1;
        }

        let (start, end) = s.split_at(end);
        TruncateContent::Middle(start.to_string(), end.to_string())
    }
}

/// Properties for [`Truncate`].
#[derive(PartialEq, Properties)]
pub struct TruncateProperties {
    pub content: TruncateContent,

    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub start_class: Classes,
    #[prop_or_default]
    pub end_class: Classes,
}

/// Truncate component
///
/// A **truncate** is a tool used to shorten numeric and non-numeric character strings, typically when the string overflows its container.
///
/// See: <https://www.patternfly.org/components/truncate>
///
/// ## Properties
///
/// Defined by [`TruncateProperties`].
#[function_component(Truncate)]
pub fn truncate(props: &TruncateProperties) -> Html {
    let class = classes!("pf-v5-c-truncate", props.class.clone());
    let start_class = classes!("pf-v5-c-truncate__start", props.start_class.clone());
    let end_class = classes!("pf-v5-c-truncate__end", props.end_class.clone());

    html!(
        <span
            {class}
            style={props.style.clone()}
            id={props.id.clone()}
        >
            {
                match &props.content {
                    TruncateContent::Default(value) => html!(
                        <span class={start_class}>{ value }</span>
                    ),
                    TruncateContent::Middle(start, end) => html!(<>
                        <span class={start_class}>{ start }</span>
                        <span class={end_class}>{ end }</span>
                    </>),
                    TruncateContent::Start(value) => html!(<>
                        <span class={end_class}>{ &value }{ "\u{200E}" }</span>
                    </>),
                }
            }
        </span>
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_mid_basic() {
        let content = "0123456789".truncate_before(5);
        assert_eq!(
            TruncateContent::Middle("01234".to_string(), "56789".to_string()),
            content
        );
    }

    #[test]
    pub fn test_mid_empty() {
        let content = "".truncate_before(5);
        assert_eq!(TruncateContent::Start("".to_string()), content);
    }

    #[test]
    pub fn test_mid_over() {
        let content = "0123456789".truncate_before(20);
        assert_eq!(TruncateContent::Start("0123456789".to_string()), content);
    }

    #[test]
    pub fn test_mid_zero() {
        let content = "0123456789".truncate_before(0);
        assert_eq!(TruncateContent::Default("0123456789".to_string()), content);
    }
}
