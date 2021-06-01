use crate::{Icon, InputGroup, TextInput, TextInputIcon};
use yew::prelude::*;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub selected: String,
    #[prop_or_default]
    pub onsearch: Callback<String>,
    #[prop_or_default]
    pub children: ChildrenWithProps<ContextSelectorItem>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Toggle,
    Close,
    Search(String),
}

pub struct ContextSelector {
    props: Props,
    link: ComponentLink<Self>,

    expanded: bool,
}

impl Component for ContextSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            expanded: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Toggle => {
                self.expanded = !self.expanded;
            }
            Self::Message::Close => {
                self.expanded = false;
            }
            Self::Message::Search(value) => {
                self.props.onsearch.emit(value);
                return false;
            }
        }
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
        let mut classes = Classes::from("pf-c-context-selector");

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        return html! {
            <div class=classes>
                <button
                    class="pf-c-context-selector__toggle"
                    aria-expanded=self.expanded
                    onclick=self.link.callback(|_|Msg::Toggle)
                >
                    <span class="pf-c-context-selector__toggle-text">{&self.props.selected}</span>
                    <span class="pf-c-context-selector__toggle-icon">{Icon::CaretDown}</span>
                </button>
                <div class="pf-c-context-selector__menu" hidden=!self.expanded>
                    <div class="pf-c-context-selector__menu-search">
                        <InputGroup>
                            <TextInput
                                onchange=self.link.callback(|v|Msg::Search(v))
                                icon=TextInputIcon::Search
                                r#type="search"/>
                        </InputGroup>
                    </div>
                    <ul class="pf-c-context-selector__menu-list">
                        { for self.props.children.iter().map(|mut item|{
                            item.props.need_close = self.link.callback(|_|Msg::Close);
                            item
                        }) }
                    </ul>
                </div>
            </div>
        };
    }
}

// item

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ItemProps {
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub(crate) need_close: Callback<()>,
}

#[derive(Clone, Copy, Debug)]
pub enum ItemMsg {
    Clicked,
}

pub struct ContextSelectorItem {
    props: ItemProps,
    link: ComponentLink<Self>,
}

impl Component for ContextSelectorItem {
    type Message = ItemMsg;
    type Properties = ItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked => {
                self.props.onclick.emit(());
                self.props.need_close.emit(());
            }
        }
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
        let classes = Classes::from("pf-c-context-selector__menu-list-item");

        return html! {
            <li>
                <button
                    class=classes
                    disabled=self.props.disabled
                    type="button"
                    onclick=self.link.callback(|_|ItemMsg::Clicked)
                    >
                    { &self.props.label }
                </button>
            </li>
        };
    }
}
