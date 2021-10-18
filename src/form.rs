use crate::{Button, Validator};
use yew::prelude::*;
use yew::web_sys::HtmlInputElement;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

pub struct Form {
    props: Props,
}

impl Component for Form {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <form novalidate=true class="pf-c-form">
                { for self.props.children.iter().map(|child|{
                        child
                }) }
            </form>
        }
    }
}

// form group

#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProps {
    pub children: Children,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub helper_text: String,
}

pub struct FormGroup {
    props: FormGroupProps,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let classes = Classes::from("pf-c-form__group");

        html! {
            <div class=classes>
                <div class="pf-c-form__group-label">

                    {if !self.props.label.is_empty() {
                        html!{
                            <div class="pf-c-form__label">
                                <span class="pf-c-form__label-text">{&self.props.label}</span>

                                {if self.props.required {
                                    html!{
                                        <span class="pf-c-form__label-required" aria-hidden="true">{"*"}</span>
                                    }
                                } else {
                                    html!{}
                                }}

                            </div>
                        }
                    } else {
                        html!{}
                    }}
                </div>

                <div class="pf-c-form__group-control">
                    { for self.props.children.iter() }
                    { if !self.props.helper_text.is_empty() {html!{
                        <p class="pf-c-form__helper-text" aria-live="polite">{ &self.props.helper_text }</p>
                    }} else {html!{}}}
                </div>
            </div>
        }
    }
}

// form control

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InputState {
    Default,
    Success,
    Warning,
    Error,
}

impl Default for InputState {
    fn default() -> Self {
        Self::Default
    }
}

impl InputState {
    pub fn convert(&self, mut classes: Classes) -> (Classes, bool) {
        let mut aria_invalid = false;
        match self {
            InputState::Default => {}
            InputState::Success => classes.push("pf-m-success"),
            InputState::Warning => classes.push("pf-m-warning"),
            InputState::Error => aria_invalid = true,
        };
        (classes, aria_invalid)
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TextInputIcon {
    None,
    Calendar,
    Clock,
    Search,
    Custom,
}

impl Default for TextInputIcon {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextInputProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub id: String,
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
    pub icon: TextInputIcon,
    #[prop_or("text".into())]
    pub r#type: String,
    #[prop_or_default]
    pub placeholder: String,

    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub validator: Validator,
}

pub struct TextInput {
    props: TextInputProps,
    link: ComponentLink<Self>,
    value: String,
    input_ref: NodeRef,
}

pub enum TextInputMsg {
    Changed(String),
    Input(String),
}

impl Component for TextInput {
    type Message = TextInputMsg;
    type Properties = TextInputProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = props.value.clone();
        Self {
            props,
            link,
            value,
            input_ref: Default::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextInputMsg::Changed(data) => {
                self.value = data.clone();
                self.props.onchange.emit(data);
            }
            TextInputMsg::Input(data) => {
                self.props.oninput.emit(data);
                if let Some(value) = self.extract_value() {
                    self.value = value.clone();
                    self.props.onchange.emit(value);
                }
                // only re-render if we have a validator
                return self.props.validator.is_custom();
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            if self.props.readonly {
                self.value = self.props.value.clone();
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-form-control");

        classes = match self.props.icon {
            TextInputIcon::None => classes,
            TextInputIcon::Search => classes.extend("pf-m-search"),
            TextInputIcon::Calendar => classes.extend(vec!["pf-m-icon", "pf-m-calendar"]),
            TextInputIcon::Clock => classes.extend(vec!["pf-m-icon", "pf-m-clock"]),
            TextInputIcon::Custom => classes.extend(vec!["pf-m-icon"]),
        };

        let (classes, aria_invalid) = self.input_state().convert(classes);

        let onchange = self.link.batch_callback(|data| match data {
            ChangeData::Value(data) => vec![TextInputMsg::Changed(data)],
            _ => vec![],
        });
        let oninput = self
            .link
            .callback(|data: InputData| TextInputMsg::Input(data.value));

        html! {
            <input
                ref=self.input_ref.clone()
                class=classes
                type=self.props.r#type
                name=self.props.name
                id=self.props.id
                required=self.props.required
                disabled=self.props.disabled
                readonly=self.props.readonly
                aria-invalid=aria_invalid
                value=self.value
                placeholder=self.props.placeholder
                onchange=onchange
                oninput=oninput
                />
        }
    }
}

impl TextInput {
    /// Extract the current value from the input element
    fn extract_value(&self) -> Option<String> {
        self.input_ref
            .cast::<HtmlInputElement>()
            .map(|input| input.value())
    }

    /// Get the effective input state
    ///
    /// This may be the result of the validator, or if none was set, the provided input state
    /// from the properties.
    fn input_state(&self) -> InputState {
        match &self.props.validator {
            Validator::Custom(validator) => validator(&self.value),
            _ => self.props.state,
        }
    }
}

//
// Text area
//

#[derive(Clone, PartialEq, Eq)]
pub enum ResizeOrientation {
    Horizontal,
    Vertical,
    Both,
}

impl Default for ResizeOrientation {
    fn default() -> Self {
        Self::Both
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TextAreaProps {
    #[prop_or_default]
    pub name: String,
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
    pub resize: ResizeOrientation,

    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub oninput: Callback<String>,

    #[prop_or_default]
    pub validator: Validator,
}

pub struct TextArea {
    props: TextAreaProps,
    link: ComponentLink<Self>,
    value: String,
    input_ref: NodeRef,
}

pub enum TextAreaMsg {
    Changed(String),
    Input(String),
}

impl Component for TextArea {
    type Message = TextAreaMsg;
    type Properties = TextAreaProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = props.value.clone();
        Self {
            props,
            link,
            value,
            input_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TextAreaMsg::Changed(data) => {
                self.value = data.clone();
                self.props.onchange.emit(data);
                false
            }
            TextAreaMsg::Input(data) => {
                self.props.oninput.emit(data);
                if let Some(value) = self.extract_value() {
                    self.value = value.clone();
                    self.props.onchange.emit(value);
                }
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let classes = Classes::from("pf-c-form-control");
        let (mut classes, aria_invalid) = self.input_state().convert(classes);

        match self.props.resize {
            ResizeOrientation::Horizontal => classes.push("pf-m-resize-horizontal"),
            ResizeOrientation::Vertical => classes.push("pf-m-resize-vertical"),
            _ => {}
        }

        let onchange = self.link.batch_callback(|data| match data {
            ChangeData::Value(data) => vec![TextAreaMsg::Changed(data)],
            _ => vec![],
        });
        let oninput = self
            .link
            .callback(|data: InputData| TextAreaMsg::Input(data.value));

        html! {
            <textarea
                ref=self.input_ref.clone()
                class=classes
                name=self.props.name
                required=self.props.required
                disabled=self.props.disabled
                readonly=self.props.readonly
                aria-invalid=aria_invalid
                value=self.props.value
                onchange=onchange
                oninput=oninput
                />
        }
    }
}

impl TextArea {
    /// Extract the current value from the input element
    fn extract_value(&self) -> Option<String> {
        self.input_ref
            .cast::<HtmlInputElement>()
            .map(|input| input.value())
    }

    /// Get the effective input state
    ///
    /// This may be the result of the validator, or if none was set, the provided input state
    /// from the properties.
    fn input_state(&self) -> InputState {
        match &self.props.validator {
            Validator::Custom(validator) => validator(&self.value),
            _ => self.props.state,
        }
    }
}

//
// Action group
//

#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProps {
    children: ChildrenWithProps<Button>,
}

pub struct ActionGroup {
    props: ActionGroupProps,
}

impl Component for ActionGroup {
    type Message = ();
    type Properties = ActionGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="pf-c-form__group pf-m-action">
                <div class="pf-c-form__actions">
                    { for self.props.children.iter() }
                </div>
            </div>
        }
    }
}

//
// Input group
//

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProps {
    children: Children,
}

pub struct InputGroup {
    props: InputGroupProps,
}

impl Component for InputGroup {
    type Message = ();
    type Properties = InputGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class="pf-c-input-group">
                { for self.props.children.iter() }
            </div>
        }
    }
}
