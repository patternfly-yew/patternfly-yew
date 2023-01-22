use crate::pagesidebar::PageSidebar;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub sidebar: ChildrenWithProps<PageSidebar>,
    #[prop_or_default]
    pub tools: Children,
    #[prop_or_default]
    pub logo: Children,
    #[prop_or_default]
    pub nav: Children,
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

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleSidebar => self.open = !self.open,
        }

        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        self.open = ctx.props().open;
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let click_callback = ctx.link().callback(|_| Msg::ToggleSidebar);

        html! {
            <div class="pf-c-page">
                <header class="pf-c-page__header">
                    <div class="pf-c-page__header-brand">

                        if !ctx.props().sidebar.is_empty() {
                            <div class="pf-c-page__header-brand-toggle">
                                <button
                                    aria-expanded={self.open.to_string()}
                                    class="pf-c-button pf-m-plain"
                                    type="button"
                                    onclick={click_callback}
                                    >
                                    <i class="fas fa-bars" aria-hidden="true"/>
                                </button>
                            </div>
                        }

                        <a href="#" class="pf-c-page__header-brand-link"> {
                            for ctx.props().logo.iter()
                        } </a>

                    </div>
                    <div class="pf-c-page__header-nav">{for ctx.props().nav.iter()}</div>
                    <div class="pf-c-page__header-tools"> { for ctx.props().tools.iter() }</div>
                </header>

                { for ctx.props().sidebar.iter().map(|mut s|{
                    let props = Rc::make_mut(&mut s.props);
                    props.open = self.open;
                    s
                }) }

                <main class="pf-c-page__main" tabindex="-1">
                    { for ctx.props().children.iter() }
                </main>
            </div>
        }
    }
}
