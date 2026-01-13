use crate::prelude::{
    AsClasses, ExtendClasses, InputState, ValidatingComponent, ValidatingComponentProperties,
    ValidationContext, focus, use_on_text_change,
};

use std::fmt::{Display, Formatter};
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
            ResizeOrientation::Both => classes.push("pf-m-resize-both"),
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
    pub name: Option<AttrValue>,
    #[prop_or_default]
    pub id: Option<AttrValue>,
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
    pub placeholder: Option<AttrValue>,
    #[prop_or_default]
    pub autofocus: bool,
    #[prop_or_default]
    pub form: Option<AttrValue>,
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,

    #[prop_or_default]
    pub spellcheck: Option<AttrValue>,
    #[prop_or_default]
    pub wrap: Wrap,

    #[prop_or_default]
    pub rows: Option<usize>,
    #[prop_or_default]
    pub cols: Option<usize>,

    #[prop_or_default]
    pub resize: ResizeOrientation,

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
/// See: <https://www.patternfly.org/components/text-area>
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

    if props.readonly {
        classes.push("pf-m-readonly");
    }

    // validation

    {
        let value = props.value.clone();
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

    html!(
        <div class={classes}>
            <textarea
                ref={input_ref}
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

                {oninput}
            />
            if props.state != InputState::Default {
                <div class="pf-v5-c-form-control__utilities">
                    <div class="pf-v5-c-form-control__icon pf-m-status">
                        {props.state.icon()}
                    </div>
                </div>
            }
        </div>
    )
}
