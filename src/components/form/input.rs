use crate::prelude::{
    focus, value, Icon, InputState, ValidatingComponent, ValidatingComponentProperties,
    ValidationContext,
};

use yew::prelude::*;
use yew::virtual_dom::VNode;

/// Properties for [`TextInput`]
#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties {
    #[prop_or_default]
    pub name: AttrValue,
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub state: InputState,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or("text".into())]
    pub r#type: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub form: AttrValue,
    #[prop_or_default]
    pub autocomplete: AttrValue,
    #[prop_or_default]
    pub inputmode: AttrValue,
    #[prop_or_default]
    pub enterkeyhint: AttrValue,

    /// This event is triggered when the element loses focus.
    #[prop_or_default]
    pub onchange: Callback<String>,
    /// This event is similar to the onchange event.
    ///
    /// The difference is that the oninput event occurs immediately after the value of an element has changed.
    ///
    /// **NOTE:** Contrary to the HTML definition of oninput, the callback provides the full value
    /// of the input element, not just the changed part.
    #[prop_or_default]
    pub oninput: Callback<String>,
    // Called when validation should occur
    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext<String>>,

    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,

    #[prop_or_default]
    pub r#ref: NodeRef,
}

impl ValidatingComponent for TextInput {
    type Value = String;
}

impl ValidatingComponentProperties<String> for TextInputProperties {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<String>>) {
        self.onvalidate = onvalidate;
    }

    fn set_input_state(&mut self, state: InputState) {
        self.state = state;
    }
}

/// Text input component
///
/// > A **text input** is used to gather free-form text from a user.
///
/// See: <https://www.patternfly.org/v4/components/text-input>
///
/// ## Properties
///
/// Defined by [`TextInputProperties].
///
/// ## Change events
///
/// The component emits changes of the input value through the `onchange` event once the
/// component looses the focus (same of plain HTML). It also emits the full input value via the
/// `oninput` event and does the same using the `onvalidate` event. This duplication is required
/// to support both change events as well as supporting the [`ValidatingComponent`] trait.
///
/// If a value is provided via the `value` property, that value must be updated through the
/// `oninput` callback. Otherwise the value will be reset immediately and the component will
/// be effectively read-only:
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::next::TextInput;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let value = use_state_eq(String::default);
///   let onchange = {
///     let value = value.clone();
///     Callback::from(move |data| value.set(data))
///   };
///
///   html!(<TextInput value={(*value).clone()}/>)
/// }
/// ```
#[function_component(TextInput)]
pub fn text_input(props: &TextInputProperties) -> Html {
    let input_ref = props.r#ref.clone();
    let mut classes = classes!("pf-v5-c-form-control");

    if props.disabled {
        classes.push("pf-m-disabled")
    }

    if props.readonly {
        classes.push("pf-m-readonly")
    }

    if props.icon.is_some() {
        classes.push("pf-m-icon");
    }

    // validation

    {
        let value = props.value.clone();
        let onvalidate = props.onvalidate.clone();
        use_effect_with_deps(
            move |()| {
                onvalidate.emit(ValidationContext {
                    value,
                    initial: true,
                });
            },
            (),
        );
    }

    let (classes, aria_invalid) = props.state.convert(classes);

    // autofocus

    {
        let autofocus = props.autofocus;
        use_effect_with_deps(
            move |input_ref| {
                if autofocus {
                    focus(input_ref)
                }
            },
            input_ref.clone(),
        );
    }

    // change events

    let onchange = use_memo(
        |(onchange, input_ref)| {
            let input_ref = input_ref.clone();
            onchange.reform(move |_: Event| value(&input_ref).unwrap_or_default())
        },
        (props.onchange.clone(), input_ref.clone()),
    );

    let oninput = use_memo(
        |(oninput, onvalidate, input_ref)| {
            let input_ref = input_ref.clone();
            let oninput = oninput.clone();
            let onvalidate = onvalidate.clone();
            Callback::from(move |_: InputEvent| {
                // get the (complete) current value
                let value = value(&input_ref).unwrap_or_default();
                oninput.emit(value.clone());
                onvalidate.emit(value.into());
            })
        },
        (
            props.oninput.clone(),
            props.onvalidate.clone(),
            input_ref.clone(),
        ),
    );

    let icon_html = if let Some(icon) = props.icon {
        Some(html!(
            <div class="pf-v5-c-form-control__icon">
                    { icon }
            </div>
        ))
    } else {
        None
    };

    let status_html = if props.state != InputState::Default {
        Some(html!(
            <div class="pf-v5-c-form-control__icon pf-m-status">
                {props.state.icon()}
            </div>
        ))
    } else {
        None
    };

    html! (
        <div class={classes}>
            <input
                ref={input_ref}
                type={&props.r#type}
                name={&props.name}
                id={&props.id}
                required={props.required}
                disabled={props.disabled}
                readonly={props.readonly}
                aria-invalid={aria_invalid.to_string()}
                value={props.value.clone()}
                placeholder={&props.placeholder}
                form={&props.form}
                autocomplete={&props.autocomplete}
                onchange={(*onchange).clone()}
                oninput={(*oninput).clone()}
                onkeydown={&props.onkeydown}
                inputmode={&props.inputmode}
                enterkeyhint={&props.enterkeyhint}
            />

                        { None::<VNode> }
            if icon_html.is_some() || status_html.is_some() {
                <div class="pf-v5-c-form-control__utilities"> // TODO: Refactor out to component
                    { icon_html }
                    { status_html }
                </div>
            }
        </div>
    )
}
