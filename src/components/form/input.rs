use crate::ouia;
use crate::prelude::{
    Icon, InputState, ValidatingComponent, ValidatingComponentProperties, ValidationContext, focus,
    use_on_text_change,
};
use crate::utils::{Ouia, OuiaComponentType, OuiaSafe};
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::VNode;

const OUIA: Ouia = ouia!("TextInput");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum TextInputType {
    Date,
    DateTimeLocal,
    Email,
    Month,
    Number,
    Password,
    Search,
    #[default]
    Text,
    Tel,
    Time,
    Url,
}

impl IntoPropValue<Option<AttrValue>> for TextInputType {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(AttrValue::Static(match self {
            Self::Date => "date",
            Self::DateTimeLocal => "datetime-local",
            Self::Email => "email",
            Self::Month => "month",
            Self::Number => "number",
            Self::Password => "password",
            Self::Search => "search",
            Self::Text => "text",
            Self::Tel => "tel",
            Self::Time => "time",
            Self::Url => "url",
        }))
    }
}

/// Properties for [`TextInput`]
#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProperties {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub value: AttrValue,
    #[prop_or_default]
    pub size: Option<AttrValue>,
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
    #[prop_or_default]
    pub r#type: TextInputType,
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub form: Option<AttrValue>,
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,
    #[prop_or_default]
    pub inputmode: Option<AttrValue>,
    #[prop_or_default]
    pub enterkeyhint: Option<AttrValue>,
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// This event is triggered when the element's value changes.
    ///
    /// **NOTE:** Contrary to the HTML definition of onchange, the callback provides the full value
    /// of the input element and fires with every keystroke.
    #[prop_or_default]
    pub onchange: Callback<String>,
    /// The element's oninput event.
    ///
    /// **NOTE:** In previous versions `oninput` behaved as does `onchange` now.
    #[prop_or_default]
    pub oninput: Callback<InputEvent>,

    // Called when validation should occur
    #[prop_or_default]
    pub onvalidate: Callback<ValidationContext<String>>,

    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,

    #[prop_or_default]
    pub onblur: Callback<FocusEvent>,

    #[prop_or_default]
    pub r#ref: NodeRef,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
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
/// See: <https://www.patternfly.org/components/text-input>
///
/// ## Properties
///
/// Defined by [`TextInputProperties].
///
/// ## Change events
///
/// The component emits changes of the input value through the `onchange` event whenever the
/// value changes It also emits the full input value via the `onvalidate` event. This duplication
/// is required to support both change events as well as supporting the [`ValidatingComponent`]
/// trait.
///
/// If a value is provided via the `value` property, that value must be updated through the
/// `onchange` callback. Otherwise the value will be reset immediately and the component will
/// be effectively read-only:
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let value = use_state_eq(String::default);
///   let onchange = use_callback(value.clone(), |new_value, value| value.set(new_value));
///
///   html!(<TextInput {onchange} value={(*value).clone()} />)
/// }
/// ```
#[function_component(TextInput)]
pub fn text_input(props: &TextInputProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let input_ref = props.r#ref.clone();
    let mut classes = classes!("pf-v5-c-form-control", props.class.clone());

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
        let value = props.value.to_string();
        let onvalidate = props.onvalidate.clone();
        use_effect_with((), move |()| {
            onvalidate.emit(ValidationContext {
                value,
                initial: true,
            });
        });
    }

    let (classes, aria_invalid) = props.state.convert(classes);

    // autofocus

    {
        let input_ref = input_ref.clone();
        use_effect_with(props.autofocus, move |autofocus| {
            if *autofocus {
                focus(&input_ref)
            }
        });
    }

    // change events
    let onchange = use_callback(
        (props.onchange.clone(), props.onvalidate.clone()),
        |new_value: String, (onchange, onvalidate)| {
            onchange.emit(new_value.clone());
            onvalidate.emit(new_value.into());
        },
    );
    let oninput = use_on_text_change(input_ref.clone(), props.oninput.clone(), onchange);

    let icon_html = props.icon.map(|icon| {
        html!(
           <div class="pf-v5-c-form-control__icon">
                { icon }
            </div>
        )
    });

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
                type={props.r#type}
                name={&props.name}
                id={&props.id}
                size={&props.size}
                required={props.required}
                disabled={props.disabled}
                readonly={props.readonly}
                aria-describedby={&props.aria_describedby}
                aria-invalid={aria_invalid.to_string()}
                value={props.value.clone()}
                placeholder={&props.placeholder}
                form={&props.form}
                autocomplete={&props.autocomplete}
                {oninput}
                onkeydown={&props.onkeydown}
                onblur={&props.onblur}
                inputmode={&props.inputmode}
                enterkeyhint={&props.enterkeyhint}
                data-ouia-component-id={(*ouia_id).clone()}
                data-ouia-component-type={props.ouia_type}
                data-ouia-safe={props.ouia_safe}
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
