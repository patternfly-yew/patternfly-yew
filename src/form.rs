use crate::Button;
use yew::prelude::*;
use yew::virtual_dom::VChild;

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

#[derive(Copy, Clone, Eq, PartialEq)]
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
}

pub struct TextInput {
    props: TextInputProps,
}

impl Component for TextInput {
    type Message = ();
    type Properties = TextInputProps;

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
        let mut classes = Classes::from("pf-c-form-control");

        classes = match self.props.icon {
            TextInputIcon::None => classes,
            TextInputIcon::Search => classes.extend("pf-m-search"),
            TextInputIcon::Calendar => classes.extend(vec!["pf-m-icon", "pf-m-calendar"]),
            TextInputIcon::Clock => classes.extend(vec!["pf-m-icon", "pf-m-clock"]),
            TextInputIcon::Custom => classes.extend(vec!["pf-m-icon"]),
        };

        let mut aria_invalid = false;
        match self.props.state {
            InputState::Default => {}
            InputState::Success => classes.push("pf-m-success"),
            InputState::Warning => classes.push("pf-m-warning"),
            InputState::Error => aria_invalid = true,
        };

        html! {
            <input
                class=classes
                type=self.props.r#type
                name=self.props.name
                required=self.props.required
                disabled=self.props.disabled
                readonly=self.props.readonly
                aria_invalid=aria_invalid
                value=self.props.value
                />
        }
    }
}

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
            { for self.props.children.iter() }
        }
    }
}
