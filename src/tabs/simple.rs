use crate::Icon;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab>,

    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub r#box: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub filled: bool,
}

pub struct Tabs {
    active: usize,
}

pub enum Msg {
    Select(usize),
}

impl Component for Tabs {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { active: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Select(idx) => {
                if self.active != idx {
                    self.active = idx;
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-tabs");

        if ctx.props().r#box {
            classes.push("pf-m-box");
        }

        if ctx.props().vertical {
            classes.push("pf-m-vertical");
        }

        if ctx.props().filled {
            classes.push("pf-m-fill");
        }

        let mut idx = 0;
        let children = ctx
            .props()
            .children
            .iter()
            .map(|mut c| {
                let props = Rc::make_mut(&mut c.props);
                props.current = self.active == idx;
                props.onselect = ctx.link().callback(move |_| Msg::Select(idx));
                idx += 1;
                c
            })
            .collect::<Vec<VChild<Tab>>>();

        let active = children[self.active].props.children.clone();

        return html! {
            <>
            <div class={classes} id={ctx.props().id.clone()}>
                <button
                    class="pf-c-tabs__scroll-button"
                    disabled=true
                    aria-hidden="true"
                    aria-label="Scroll left"
                    >
                    { Icon::AngleLeft }
                </button>
                <ul class="pf-c-tabs__list">
                    { for children.into_iter() }
                </ul>
                <button
                    class="pf-c-tabs__scroll-button"
                    disabled=true
                    aria-hidden="true"
                    aria-label="Scroll right"
                >
                    { Icon::AngleRight }
                </button>
            </div>
            { active }
            </>
        };
    }
}

// tab

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabProps {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<Icon>,

    #[prop_or_default]
    pub(crate) onselect: Callback<()>,
    #[prop_or_default]
    pub(crate) current: bool,

    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, Copy, Debug)]
pub enum TabMsg {
    Clicked,
}

pub struct Tab {}

impl Component for Tab {
    type Message = TabMsg;
    type Properties = TabProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabMsg::Clicked => ctx.props().onselect.emit(()),
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-tabs__item");

        if ctx.props().current {
            classes.push("pf-m-current");
        }

        return html! {
            <li class={classes}>
                <button class="pf-c-tabs__link" onclick={ctx.link().callback(|_|TabMsg::Clicked)}>
                    if let Some(icon) = ctx.props().icon {
                        <span class="pf-c-tabs__item-icon" aria_hidden={true.to_string()}> { icon } </span>
                    }
                    <span class="pf-c-tabs__item-text"> { &ctx.props().label } </span>
                </button>
            </li>
        };
    }
}
