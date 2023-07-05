use yew::{html::IntoPropValue, prelude::*, virtual_dom::VNode};

/// Wrap an element with style.
pub struct Styled<T>
where
    T: Into<Html>,
{
    pub content: T,
    pub style: Option<AttrValue>,
}

impl<T> Styled<T>
where
    T: Into<Html>,
{
    pub fn into_html(self) -> Html {
        html! (
            <span style={&self.style}>
                { self.content.into() }
            </span>
        )
    }
}

impl<T> From<Styled<T>> for VNode
where
    T: Into<Html>,
{
    fn from(value: Styled<T>) -> Self {
        value.into_html()
    }
}

impl<T> IntoPropValue<Html> for Styled<T>
where
    T: Into<Html>,
{
    fn into_prop_value(self) -> Html {
        self.into_html()
    }
}

impl<T> IntoPropValue<Option<Html>> for Styled<T>
where
    T: Into<Html>,
{
    fn into_prop_value(self) -> Option<Html> {
        Some(self.into_html())
    }
}
