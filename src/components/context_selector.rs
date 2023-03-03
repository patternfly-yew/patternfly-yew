//! Context selector
use crate::{GlobalClose, Icon, InputGroup, TextInput, TextInputIcon};
use std::rc::Rc;
use yew::prelude::*;

/// Properties for [`ContextSelector`]
#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ContextSelectorProperties {
    #[prop_or_default]
    pub selected: String,
    #[prop_or_default]
    pub onsearch: Callback<String>,
    #[prop_or_default]
    pub children: ChildrenWithProps<ContextSelectorItem>,
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum Msg {
    Toggle,
    Close,
    Search(String),
}

/// Context Selector component
///
/// > A **context selector** can be used in addition to global navigation when the data or resources you show in the interface need to change depending on the user's context.
///
/// See: <https://www.patternfly.org/v4/components/context-selector>
///
/// ## Properties
///
/// Defined by [`ContextSelectorProperties`].
pub struct ContextSelector {
    expanded: bool,
    global_close: GlobalClose,
}

impl Component for ContextSelector {
    type Message = Msg;
    type Properties = ContextSelectorProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let global_close =
            GlobalClose::new(NodeRef::default(), ctx.link().callback(|_| Msg::Close));
        Self {
            expanded: false,
            global_close,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Toggle => {
                self.expanded = !self.expanded;
            }
            Self::Message::Close => {
                self.expanded = false;
            }
            Self::Message::Search(value) => {
                ctx.props().onsearch.emit(value);
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-context-selector");

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        html! (
            <div
                class={classes}
                ref={self.global_close.clone()}
            >
                <button
                    class="pf-c-context-selector__toggle"
                    aria-expanded={self.expanded.to_string()}
                    type="button"
                    onclick={ctx.link().callback(|_|Msg::Toggle)}
                >
                    <span class="pf-c-context-selector__toggle-text">{&ctx.props().selected}</span>
                    <span class="pf-c-context-selector__toggle-icon">{Icon::CaretDown}</span>
                </button>
                <div class="pf-c-context-selector__menu"
                    hidden={!self.expanded}
                >
                    <div class="pf-c-context-selector__menu-search">
                        <InputGroup>
                            <TextInput
                                onchange={ctx.link().callback(Msg::Search)}
                                icon={TextInputIcon::Search}
                                r#type="search"/>
                        </InputGroup>
                    </div>
                    <ul class="pf-c-context-selector__menu-list">
                        { for ctx.props().children.iter().map(|mut item|{
                            let mut props = Rc::make_mut(&mut item.props);
                            props.need_close = ctx.link().callback(|_|Msg::Close);
                            item
                        }) }
                    </ul>
                </div>
            </div>
        )
    }
}

// item

/// Properties for [`ContextSelectorItem`]
#[derive(Properties, Debug, Clone, PartialEq)]
pub struct ItemProperties {
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<()>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub(crate) need_close: Callback<()>,
}

#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub enum ItemMsg {
    Clicked,
}

/// An item of a [`ContextSelector`]
pub struct ContextSelectorItem {}

impl Component for ContextSelectorItem {
    type Message = ItemMsg;
    type Properties = ItemProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::Clicked => {
                ctx.props().onclick.emit(());
                ctx.props().need_close.emit(());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let classes = Classes::from("pf-c-context-selector__menu-list-item");

        html!(
            <li>
                <button
                    class={classes}
                    disabled={ctx.props().disabled}
                    type="button"
                    onclick={ctx.link().callback(|_|ItemMsg::Clicked)}
                    >
                    { &ctx.props().label }
                </button>
            </li>
        )
    }
}
