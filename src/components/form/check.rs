use crate::hooks::id::use_prop_id;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for [`Check`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CheckProperties {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub required: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub onchange: Callback<bool>,
}

/// Checkbox component
///
/// ## Properties
///
/// Defined by [`CheckProperties`].
#[function_component(Check)]
pub fn check(props: &CheckProperties) -> Html {
    let id = use_prop_id(props.id.clone());

    let mut class = classes!("pf-v5-c-check");
    let mut label_class = classes!("pf-v5-c-check__label");

    if props.disabled {
        label_class.extend(classes!("pf-m-disabled"));
    }

    let standalone = props.children.is_empty();
    if standalone {
        class.extend(classes!("pf-m-standalone"));
    }

    let input_ref = use_node_ref();

    let onchange = {
        let input_ref = input_ref.clone();
        let onchange = props.onchange.clone();
        Callback::from(move |_| {
            let state = input_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.checked())
                .unwrap_or_default();
            onchange.emit(state);
        })
    };

    html!(
        <div {class}>
            <input
                ref={input_ref}
                class="pf-v5-c-check__input"
                type="checkbox"
                id={(*id).clone()}
                name={&props.name}
                disabled={props.disabled}
                checked={props.checked}
                {onchange}
            />

            if !standalone {
                <label class={label_class} for={(*id).clone()}>
                    { props.children.clone() }
                    if props.required {
                        <span class="pf-v5-c-check__label-required" aria-hidden="true"> { "&#42;" } </span>
                    }
                </label>
            }

        </div>
    )
}
