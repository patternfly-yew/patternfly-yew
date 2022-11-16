use crate::{Icon, Spinner, SpinnerSize};
use web_sys::HtmlElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    None,
    Primary,
    Secondary,
    Tertiary,
    Warning,
    Danger,
    DangerSecondary,
    Link,
    InlineLink,
    Control,
    Plain,
}

impl Variant {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Variant::None => vec![],
            Variant::Primary => vec!["pf-m-primary"],
            Variant::Secondary => vec!["pf-m-secondary"],
            Variant::Tertiary => vec!["pf-m-tertiary"],
            Variant::Warning => vec!["pf-m-warning"],
            Variant::Danger => vec!["pf-m-danger"],
            Variant::DangerSecondary => vec!["pf-m-danger", "pf-m-secondary"],
            Variant::Link => vec!["pf-m-link"],
            Variant::InlineLink => vec!["pf-m-link", "pf-m-inline"],
            Variant::Control => vec!["pf-m-control"],
            Variant::Plain => vec!["pf-m-plain"],
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::None
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Align {
    Start,
    End,
}

impl Default for Align {
    fn default() -> Self {
        Align::Start
    }
}

/// https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ButtonType {
    Submit,
    Reset,
    Button
}

impl Default for ButtonType {
    fn default() -> Self {
        ButtonType::Button
    }
}

impl ToString for ButtonType {
    fn to_string(&self) -> String {
        match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        }
            .into()
    }
}

impl IntoPropValue<Option<AttrValue>> for ButtonType {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.to_string().into())
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<yew::MouseEvent>,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub block: bool,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_default]
    pub aria_label: Option<String>,

    #[prop_or_default]
    pub r#type: ButtonType,

    #[prop_or_default]
    pub form: Option<String>,
    #[prop_or_default]
    pub formaction: Option<String>,

    #[prop_or_default]
    pub role: Option<String>,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub children: Children,
}

pub struct Button {
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(evt) => {
                ctx.props().onclick.emit(evt);
                // blur the button after a click, otherwise it will continue appear hovered/pressed
                self.blur();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let mut classes: Classes = classes!(
            "pf-c-button",
            ctx.props().class.clone(),
            ctx.props().variant.as_classes());

        if ctx.props().expanded {
            classes.push("pf-m-expanded");
        }
        if ctx.props().block {
            classes.push("pf-m-block");
        }
        if ctx.props().loading {
            classes.push("pf-m-progress pf-m-in-progress")
        }

        return html! {
            <button
                ref={self.node_ref.clone()}
                id={ctx.props().id.clone()}
                class={classes}
                style={ctx.props().style.as_ref().cloned().unwrap_or_default()}
                disabled={ctx.props().disabled}
                type={ctx.props().r#type.clone()}
                onclick={ctx.link().callback(Msg::Clicked)}
                role={ctx.props().role.clone().unwrap_or_default()}
                form={ctx.props().form.clone()}
                formaction={ctx.props().formaction.clone()}
            >
                if ctx.props().loading {
                    <span class="pf-c-button__progress">
                        <Spinner size={SpinnerSize::Md} />
                    </span>
                }

                { self.label(ctx) }
                { for ctx.props().children.iter() }

            </button>
        };
    }
}

impl Button {
    fn icon(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-button__icon");

        match ctx.props().align {
            Align::Start => classes.push("pf-m-start"),
            Align::End => classes.push("pf-m-end"),
        }

        match ctx.props().icon {
            Some(i) => html! {
                <span class={classes}>
                    { i }
                </span>
            },
            None => html! {},
        }
    }

    fn label(&self, ctx: &Context<Self>) -> Vec<Html> {
        let label = ctx.props().label.clone().into();
        match ctx.props().align {
            Align::Start => vec![self.icon(ctx), label],
            Align::End => vec![label, self.icon(ctx)],
        }
    }

    /// Blur (loose focus) on the button element
    fn blur(&self) {
        if let Some(node) = self.node_ref.cast::<HtmlElement>() {
            node.blur().ok();
        }
    }
}
