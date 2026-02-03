//! Text Input Group

use crate::{prelude::use_on_text_change, utils::HtmlElementSupport};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputGroupProperties {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub disabled: bool,
}

/// Text input group component
///
/// > A **text input** group is a more flexible composable version of a text input. It enables consumers of PatternFly to build custom inputs for filtering and similar use cases by placing elements like icons, chips groups and buttons within a text input.
///
/// See: <https://www.patternfly.org/components/text-input-group>
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
    let mut class = classes!("pf-v6-c-text-input-group");

    class.extend(props.class.clone());

    if props.disabled {
        class.extend(classes!("pf-m-disabled"));
    }

    html!(
        <div {class} id={&props.id} style={&props.style}>
            { props.children.clone() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputGroupMainProperties {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// The value of the input component
    #[prop_or_default]
    pub value: String,

    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    #[prop_or_default]
    pub icon: Option<Html>,

    /// Disables the component
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    #[prop_or_default]
    pub onchange: Callback<String>,

    #[prop_or_default]
    pub oninput: Callback<InputEvent>,

    #[prop_or_default]
    pub r#type: Option<AttrValue>,

    #[prop_or_default]
    pub inputmode: Option<AttrValue>,

    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,

    #[prop_or_default]
    pub autofocus: bool,

    #[prop_or_default]
    pub inner_ref: Option<NodeRef>,

    #[prop_or_default]
    pub hint: Option<AttrValue>,
}

#[function_component(TextInputGroupMain)]
pub fn text_input_group_main(props: &TextInputGroupMainProperties) -> Html {
    let mut class = classes!("pf-v6-c-text-input-group__main");
    class.extend(props.class.clone());

    if props.icon.is_some() {
        class.push(classes!("pf-m-icon"));
    }

    let node_ref = use_node_ref();
    let node_ref = props.inner_ref.as_ref().unwrap_or(&node_ref);
    let oninput = use_on_text_change(
        node_ref.clone(),
        props.oninput.clone(),
        props.onchange.clone(),
    );

    // autofocus

    {
        let node_ref = node_ref.clone();
        use_effect_with(props.autofocus, move |autofocus| {
            if *autofocus {
                node_ref.focus();
            }
        });
    }

    // render

    html!(
        <div
            {class}
            id={&props.id}
            style={&props.style}
        >
            <span class="pf-v6-c-text-input-group__text">
                if let Some(hint) = &props.hint {
                    <input
                        class="pf-v6-c-text-input-group__text-input pf-m-hint"
                        type="text"
                        disabled=true
                        aria-hidden="true"
                        value={hint}
                    />
                }
                if let Some(icon) = &props.icon {
                    <span class="pf-v6-c-text-input-group__icon">
                        { icon.clone() }
                    </span>
                }
                <input
                    class="pf-v6-c-text-input-group__text-input"
                    ref={node_ref}
                    type={&props.r#type}
                    inputmode={&props.inputmode}
                    {oninput}
                    disabled={props.disabled}
                    placeholder={&props.placeholder}
                    value={props.value.clone()}
                    aria-label={&props.aria_label}
                    onkeydown={&props.onkeydown}
                />
            </span>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TextInputGroupUtilitiesProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(TextInputGroupUtilities)]
pub fn text_input_group_utilities(props: &TextInputGroupUtilitiesProperties) -> Html {
    html! (
        <div class="pf-v6-c-text-input-group__utilities">
            { props.children.clone() }
        </div>
    )
}
