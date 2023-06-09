use crate::{
    focus, value, AsClasses, ExtendClasses, InputState, ValidatingComponent,
    ValidatingComponentProperties, ValidationContext, Validator,
};

use std::fmt::{Display, Formatter};
use web_sys::HtmlInputElement;
use yew::prelude::*;

//
// Text area
//

#[derive(Clone, Default, PartialEq, Eq)]
pub enum ResizeOrientation {
    Horizontal,
    Vertical,
    #[default]
    Both,
}

impl AsClasses for ResizeOrientation {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            ResizeOrientation::Horizontal => classes.push("pf-m-resize-horizontal"),
            ResizeOrientation::Vertical => classes.push("pf-m-resize-vertical"),
            ResizeOrientation::Both => {}
        }
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
pub enum Wrap {
    Hard,
    #[default]
    Soft,
    Off,
}

impl Display for Wrap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Off => f.write_str("off"),
            Self::Soft => f.write_str("soft"),
            Self::Hard => f.write_str("hard"),
        }
    }
}

/// Properties for [`TextArea`]
#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProperties {
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
    pub placeholder: AttrValue,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub form: AttrValue,
    #[prop_or_default]
    pub autocomplete: AttrValue,

    #[prop_or_default]
    pub spellcheck: AttrValue,
    #[prop_or_default]
    pub wrap: Wrap,

    #[prop_or_default]
    pub rows: Option<usize>,
    #[prop_or_default]
    pub cols: Option<usize>,

    #[prop_or_default]
    pub resize: ResizeOrientation,

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
    pub r#ref: NodeRef,
}

impl ValidatingComponent for TextArea {
    type Value = String;
}

impl ValidatingComponentProperties<String> for TextAreaProperties {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<String>>) {
        self.onvalidate = onvalidate;
    }

    fn set_input_state(&mut self, state: InputState) {
        self.state = state;
    }
}

/// Text area component
///
/// > A **text area** component is used for entering a paragraph of text that is longer than one line.
///
/// See: <https://www.patternfly.org/v4/components/text-area>
///
/// ## Properties
///
/// Defined by [`TextAreaProperties].
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
///   html!(<TextArea value={(*value).clone()}/>)
/// }
/// ```
#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProperties) -> Html {
    let input_ref = props.r#ref.clone();
    let mut classes = classes!("pf-v5-c-form-control");

    classes.extend_from(&props.resize);

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

    html!(
        <textarea
            ref={input_ref}
            class={classes}
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

            cols={props.cols.as_ref().map(|v|v.to_string())}
            rows={props.rows.as_ref().map(|v|v.to_string())}

            wrap={props.wrap.to_string()}
            spellcheck={&props.spellcheck}

            onchange={(*onchange).clone()}
            oninput={(*oninput).clone()}
        />
    )
}
