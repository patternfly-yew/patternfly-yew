use crate::logo::Logo;
use crate::pagesidebar::PageSidebar;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::VChild;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub sidebar: Option<VChild<PageSidebar>>,
    #[prop_or_default]
    pub tools: Children,
    #[prop_or_default]
    pub logo: Option<VChild<Logo>>,
    #[prop_or(true)]
    pub open: bool,
}

pub struct Page {
    open: bool,
}

pub enum Msg {
    ToggleSidebar,
}

impl Component for Page {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            open: ctx.props().open,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        self.open = ctx.props().open;
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSidebar => self.open = !self.open,
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="pf-c-page">
                <header class="pf-c-page__header">
                    <div class="pf-c-page__header-brand">
                        { self.sidebar_button(ctx) }
                        <a href="#" class="pf-c-page__header-brand-link">{
                            ctx.props().logo.clone().map(Html::from).unwrap_or_default()
                        }</a>
                    </div>
                    <div class="pf-c-page__header-tools"> { for ctx.props().tools.iter() }</div>
                </header>

                { ctx.props().sidebar.clone().map(|mut s|{
                    let props = Rc::make_mut(&mut s.props);
                    props.open = self.open;
                    s
                }).map(Html::from).unwrap_or_default() }

                <main class="pf-c-page__main" tabindex="-1">
                    { for ctx.props().children.iter() }
                </main>
            </div>
        }
    }
}

impl Page {
    fn sidebar_button(&self, ctx: &Context<Self>) -> Html {
        let click_callback = ctx.link().callback(|_| Msg::ToggleSidebar);

        match &ctx.props().sidebar {
            Some(_) => html! {<div class="pf-c-page__header-brand-toggle">
                <button
                    aria-expanded={ctx.props().sidebar.as_ref().map(|sidebar|sidebar.props.open).unwrap_or(false).to_string()}
                    class="pf-c-button pf-m-plain"
                    type="button"
                    onclick={click_callback}
                    >
                    <i class="fas fa-bars" aria-hidden="true"/>
                </button>
            </div>},
            None => html! {},
        }
    }
}
