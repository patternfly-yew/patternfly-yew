use crate::prelude::attr_value_to_static_cow;
use implicit_clone::unsync::IString;
use yew::prelude::*;

/// Properties for [`Visible`].
#[derive(PartialEq, Properties)]
pub struct VisibleProperties {
    /// The visibility flag
    pub visible: bool,

    /// The content to show/hide
    #[prop_or_default]
    pub children: Html,

    /// Additional classes
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// The element to wrap the content with. Defaults to `div`.
    #[prop_or("div".into())]
    pub element: IString,
}

/// A component hiding its content when not being visible.
///
/// While the content is still part of the DOM tree, the wrapping element (and its children)
/// will not be visible.
///
/// ## Properties
///
/// Defined by [`VisibleProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let visible = true;
///   html!(
///     <>
///       <Visible {visible}>
///         { "Show me" }
///       </Visible>
///       <Visible {visible} element="span">
///         { "Show me (inline)" }
///       </Visible>
///     </>
///   )
/// }
/// ```
#[function_component(Visible)]
pub fn visible(props: &VisibleProperties) -> Html {
    let mut class = match props.visible {
        true => classes!(),
        false => classes!("pf-v6-u-display-none"),
    };

    class.extend(&props.class);

    html!(
        <@{attr_value_to_static_cow(&props.element)}
            {class}
            style={props.style.clone()}
            id={props.id.clone()}
        >
            { props.children.clone() }
        </@>
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ele() {
        let element: String = "div".into();
        let _ = html!(<Visible visible={true} {element} / >);
        let _ = html!(<Visible visible={true} element={"div"} / >);
    }
}
