use crate::{Avatar, Button, Divider, GlobalClose, Icon, Position, Variant};
use std::rc::Rc;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub position: Position,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub plain: bool,

    pub toggle: Html,
    #[prop_or_default]
    pub toggle_style: Option<String>,

    #[prop_or_default]
    pub children: ChildrenRenderer<DropdownChildVariant>,
}

pub struct Dropdown {
    expanded: bool,
    global_close: GlobalClose,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Toggle,
    Close,
}

impl Component for Dropdown {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            expanded: false,
            global_close: GlobalClose::new(NodeRef::default(), ctx.link().callback(|_| Msg::Close)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
            Msg::Close => self.expanded = false,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-dropdown");
        if self.expanded {
            classes.push("pf-m-expanded");
        }

        let mut menu_classes = Classes::from("pf-c-dropdown__menu");

        match ctx.props().position {
            Position::Left => {}
            Position::Right => menu_classes.push("pf-m-align-right"),
            Position::Top => classes.push("pf-m-top"),
        }

        let onclick = ctx.link().callback(|_| Msg::Toggle);

        let variant = match ctx.props().plain {
            true => Variant::Plain,
            false => Variant::None,
        };

        html! {
            <div class={classes}
                ref={self.global_close.clone()}
            >
                <Button
                    class="pf-c-dropdown__toggle"
                    style={ctx.props().toggle_style.clone()}
                    variant={variant}
                    r#type="button"
                    disabled={ctx.props().disabled}
                    onclick={onclick}
                    id={ctx.props().id.clone()}
                    >
                    { ctx.props().toggle.clone() }
                </Button>
                <div
                    class={menu_classes}
                    hidden={!self.expanded}
                >
                    <ul>
                    { for ctx.props().children.iter().map(|mut c|{
                        // request a close callback from the item
                        c.set_need_close(ctx.link().callback(|_|Msg::Close));
                        c
                    }) }
                    </ul>
                </div>
            </div>
        }
    }
}

// toggle

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownToggleProps {
    #[prop_or_default]
    pub image: Option<Html>,
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub icon: Option<Icon>,
}

#[function_component(DropdownToggle)]
pub fn dropdown_toggle(props: &DropdownToggleProps) -> Html {
    html! {
        <>
            if let Some(image) = &props.image {
                <span class="pf-c-dropdown__toggle-image">
                    { image.clone() }
                </span>
            }
            <span class="pf-c-dropdown__toggle-text">
                { &props.text }
            </span>
            <span class="pf-c-dropdown__toggle-icon">
                if let Some(icon) = props.icon {
                    { icon }
                } else {
                    { Icon::CaretDown }
                }
            </span>
        </>
    }
}

// child

#[derive(Clone, PartialEq)]
pub enum DropdownChild {
    Item(Rc<<DropdownItem as Component>::Properties>),
    Divider(Rc<<Divider as Component>::Properties>),
    Group(Rc<<DropdownItemGroup as Component>::Properties>),
    Text(Rc<<DropdownItemText as Component>::Properties>),
}

impl From<DropdownItemProps> for DropdownChild {
    fn from(props: DropdownItemProps) -> Self {
        DropdownChild::Item(Rc::new(props))
    }
}

impl From<()> for DropdownChild {
    fn from(_: ()) -> Self {
        DropdownChild::Divider(Rc::new(()))
    }
}

impl From<DropdownItemGroupProps> for DropdownChild {
    fn from(props: DropdownItemGroupProps) -> Self {
        DropdownChild::Group(Rc::new(props))
    }
}

impl From<DropdownItemTextProps> for DropdownChild {
    fn from(props: DropdownItemTextProps) -> Self {
        DropdownChild::Text(Rc::new(props))
    }
}

// variant

#[derive(PartialEq, Clone)]
pub struct DropdownChildVariant {
    props: DropdownChild,
}

impl DropdownChildVariant {
    /// Forward the need to get a close callback to the actual item
    fn set_need_close(&mut self, callback: Callback<()>) {
        match self.props {
            DropdownChild::Item(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_close = callback;
            }
            DropdownChild::Group(ref mut props) => {
                let props = Rc::make_mut(props);
                props.want_close = callback;
            }
            _ => {}
        }
    }
}

impl<CHILD> From<VChild<CHILD>> for DropdownChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<DropdownChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl Into<Html> for DropdownChildVariant {
    fn into(self) -> Html {
        match self.props {
            DropdownChild::Item(props) => {
                VComp::new::<DropdownItem>(props, NodeRef::default(), None).into()
            }
            DropdownChild::Divider(props) => {
                VComp::new::<Divider>(props, NodeRef::default(), None).into()
            }
            DropdownChild::Group(props) => {
                VComp::new::<DropdownItemGroup>(props, NodeRef::default(), None).into()
            }
            DropdownChild::Text(props) => {
                VComp::new::<DropdownItemText>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
    #[prop_or_default]
    pub(crate) want_close: Callback<()>,
}

#[derive(Clone, Copy, Debug)]
pub enum DropdownItemMsg {
    Clicked,
}

#[derive(Clone)]
pub struct DropdownItem {}

impl Component for DropdownItem {
    type Message = DropdownItemMsg;
    type Properties = DropdownItemProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked => {
                if let Some(onclick) = &ctx.props().onclick {
                    onclick.emit(());
                }
                // request close from our parent
                ctx.props().want_close.emit(());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let action = if ctx.props().onclick.is_some() {
            html! {
                <Button
                    class="pf-c-dropdown__menu-item"
                    onclick={ctx.link().callback(|_|Self::Message::Clicked)}
                    >
                    { for ctx.props().children.iter() }
                </Button>
            }
        } else {
            html! {
                <a
                    class="pf-c-dropdown__menu-item"
                    target={ctx.props().target.clone()}
                    href={ctx.props().href.clone()}>{ for ctx.props().children.iter() }</a>
            }
        };

        return html! {
            <li>{action}</li>
        };
    }
}

// Group

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemGroupProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<DropdownChildVariant>,
    #[prop_or_default]
    pub(crate) want_close: Callback<()>,
}

#[derive(Clone)]
pub struct DropdownItemGroup {}

#[derive(Clone, Copy, Debug)]
pub enum DropdownItemGroupMsg {
    Close,
}

impl Component for DropdownItemGroup {
    type Message = DropdownItemGroupMsg;
    type Properties = DropdownItemGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Close => ctx.props().want_close.emit(()),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
            { for ctx.props().children.iter().map(|mut c|{
                c.set_need_close(ctx.link().callback(|_|Self::Message::Close));
                html!{
                    <section class="pf-c-dropdown__group">
                    { c }
                    </section>
                }
            })}
            </>
        }
    }
}

// Text

#[derive(Clone, PartialEq, Properties)]
pub struct DropdownItemTextProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DropdownItemText)]
pub fn dropwdown_item_text(props: &DropdownItemTextProps) -> Html {
    html! {
        <div class="pf-c-dropdown__menu-item pf-m-text">
        { for props.children.iter() }
        </div>
    }
}

// kebab toggle

#[function_component(KebabToggle)]
pub fn kebab_toggle() -> Html {
    html! {
        <DropdownToggle icon={Icon::EllipsisV}/>
    }
}

// user toggle

#[derive(Clone, PartialEq, Properties)]
pub struct UserToggleProps {
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub src: String,
}

#[function_component(UserToggle)]
pub fn user_toggle(props: &UserToggleProps) -> Html {
    let image = html! { <Avatar src={props.src.clone()} /> };
    html! {
        <DropdownToggle
            image={image}
            text={props.name.clone()}
            />
    }
}
