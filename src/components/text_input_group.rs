//! Text Input Group

use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputGroupProperties {
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub children: Children,
}

/// Text input group component
///
/// > A **text input** group is a more flexible composable version of a text input. It enables consumers of PatternFly to build custom inputs for filtering and similar use cases by placing elements like icons, chips groups and buttons within a text input.
///
/// See: <https://www.patternfly.org/v4/components/text-input-group>
///
/// ## Properties
///
/// Defined by [`TextInputGroupProperties`].
///
/// ## Children
///
/// This component is mainly a container, it requires one [`TextInputGroupMain`] to work properly.
#[function_component(TextInputGroup)]
pub fn text_input_group(props: &TextInputGroupProperties) -> Html {
    html!(
        <div class="pf-c-text-input-group" id={&props.id}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputGroupMainProperties {
    /// The value of the input component
    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub placeholder: AttrValue,

    #[prop_or_default]
    pub icon: Option<Html>,

    /// Disables the component
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub oninput: Callback<String>,
}

#[function_component(TextInputGroupMain)]
pub fn text_input_group_main(props: &TextInputGroupMainProperties) -> Html {
    let mut class = classes!("pf-c-text-input-group__main");

    if props.icon.is_some() {
        class.push(classes!("pf-m-icon"));
    }

    let node_ref = use_node_ref();

    let oninput = {
        let node_ref = node_ref.clone();
        let onchange = props.oninput.clone();
        Callback::from(move |_| {
            if let Some(input) = node_ref.cast::<HtmlInputElement>() {
                onchange.emit(input.value());
            }
        })
    };

    html!(
        <div {class}>
            <span class="pf-c-text-input-group__text">
                if let Some(icon) = &props.icon {
                    <span class="pf-c-text-input-group__icon">
                        { icon.clone() }
                    </span>
                }
                <input
                    class="pf-c-text-input-group__text-input"
                    ref={node_ref}
                    type="text"
                    {oninput}
                    disabled={props.disabled}
                    placeholder={&props.placeholder}
                    value={props.value.clone()}
                    aria-label={&props.aria_label}
                />
            </span>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputGroupUtilitiesProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(TextInputGroupUtilities)]
pub fn text_input_group_utilities(props: &TextInputGroupUtilitiesProperties) -> Html {
    html! (
        <div class="pf-c-text-input-group__utilities">
            { for props.children.iter() }
        </div>
    )
}
