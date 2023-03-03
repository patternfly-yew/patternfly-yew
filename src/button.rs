use crate::{Icon, Spinner, SpinnerSize};
use web_sys::HtmlElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[deprecated(since = "0.4.0", note = "Got renamed to 'ButtonVariant'")]
pub type Variant = ButtonVariant;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
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

impl ButtonVariant {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Self::None => vec![],
            Self::Primary => vec!["pf-m-primary"],
            Self::Secondary => vec!["pf-m-secondary"],
            Self::Tertiary => vec!["pf-m-tertiary"],
            Self::Warning => vec!["pf-m-warning"],
            Self::Danger => vec!["pf-m-danger"],
            Self::DangerSecondary => vec!["pf-m-danger", "pf-m-secondary"],
            Self::Link => vec!["pf-m-link"],
            Self::InlineLink => vec!["pf-m-link", "pf-m-inline"],
            Self::Control => vec!["pf-m-control"],
            Self::Plain => vec!["pf-m-plain"],
        }
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

/// Button Type.
///
/// See: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ButtonType {
    Submit,
    Reset,
    Button,
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
pub struct ButtonProperties {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: Option<String>,

    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub variant: ButtonVariant,
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

/// Button component
///
/// > A **button** is a box area or text that communicates and triggers user actions when clicked or selected. Buttons can be used to communicate and immediately trigger actions a user can take in an application, like submitting a form, canceling a process, or creating a new object. Buttons can also be used to take a user to a new location, like another page inside of a web application, or an external site such as help or documentation.
///
/// See: <https://www.patternfly.org/v4/components/button>
///
/// ## Properties
///
/// Defined by [`ButtonProperties`].
pub struct Button {
    node_ref: NodeRef,
}

pub enum Msg {
    Clicked(MouseEvent),
}

impl Component for Button {
    type Message = Msg;
    type Properties = ButtonProperties;

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
            ctx.props().variant.as_classes()
        );

        if ctx.props().expanded {
            classes.push("pf-m-expanded");
        }
        if ctx.props().block {
            classes.push("pf-m-block");
        }
        if ctx.props().loading {
            classes.push("pf-m-progress pf-m-in-progress")
        }

        html! (
             <button
                 ref={self.node_ref.clone()}
                 id={ctx.props().id.clone()}
                 class={classes}
                 style={ctx.props().style.clone()}
                 disabled={ctx.props().disabled}
                 type={ctx.props().r#type}
                 onclick={ctx.link().callback(Msg::Clicked)}
                 role={ctx.props().role.clone()}
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
        )
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
