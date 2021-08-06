use crate::Icon;
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
    props: Props,
    link: ComponentLink<Self>,
    active: usize,
}

pub enum Msg {
    Select(usize),
}

impl Component for Tabs {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            active: 0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
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

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-tabs");

        if self.props.r#box {
            classes = classes.extend("pf-m-box");
        }

        if self.props.vertical {
            classes = classes.extend("pf-m-vertical");
        }

        if self.props.filled {
            classes = classes.extend("pf-m-fill");
        }

        let mut idx = 0;
        let children = self
            .props
            .children
            .iter()
            .map(|mut c| {
                c.props.current = self.active == idx;
                c.props.onselect = self.link.callback(move |_| Msg::Select(idx));
                idx += 1;
                c
            })
            .collect::<Vec<VChild<Tab>>>();

        let active = children[self.active].props.children.clone();

        return html! {
            <>
            <div class=classes id=self.props.id>
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

pub struct Tab {
    props: TabProps,
    link: ComponentLink<Self>,
}

impl Component for Tab {
    type Message = TabMsg;
    type Properties = TabProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TabMsg::Clicked => self.props.onselect.emit(()),
        }
        false
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
        let mut classes = Classes::from("pf-c-tabs__item");

        if self.props.current {
            classes = classes.extend("pf-m-current");
        }

        return html! {
            <li class=classes>
                <button class="pf-c-tabs__link" onclick=self.link.callback(|_|TabMsg::Clicked)>
                    {
                        if let Some(icon) = self.props.icon {
                            html!{
                                <span class="pf-c-tabs__item-icon" aria_hidden=true> { icon } </span>
                            }
                        } else {
                            html!{}
                        }
                    }
                    <span class="pf-c-tabs__item-text"> { &self.props.label } </span>
                </button>
            </li>
        };
    }
}
