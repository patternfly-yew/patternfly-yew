use crate::{AsClasses, ExtendClasses, Icon, Inset, WithBreakpoints};
use std::rc::Rc;
use yew::prelude::*;

/// Properties for [`Tabs`]
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProperties {
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

    #[prop_or_default]
    pub inset: Option<TabInset>,

    /// Enable "detached" mode
    ///
    /// If enabled, the content of tabs will not be rendered.
    #[prop_or_default]
    pub detached: bool,
    #[prop_or_default]
    pub onselect: Callback<usize>,

    /// Set the current active tab, overrides the internal state.
    #[prop_or_default]
    pub active: Option<usize>,
}

/// Tabs component
///
/// > **Tabs** allow users to navigate between views within the same page or context.
///
/// See: <https://www.patternfly.org/v4/components/tabs>
///
/// ## Properties
///
/// Defined by [`TabsProperties`].
#[function_component(Tabs)]
pub fn tabs(props: &TabsProperties) -> Html {
    let active = use_state_eq(|| props.active.unwrap_or_default());

    let mut classes = classes!("pf-c-tabs");

    if props.r#box {
        classes.push("pf-m-box");
    }

    if props.vertical {
        classes.push("pf-m-vertical");
    }

    if props.filled {
        classes.push("pf-m-fill");
    }

    classes.extend_from(&props.inset);

    html! (
        <>
            <div
                class={classes}
                id={props.id.clone()}
            >
                <button
                    class="pf-c-tabs__scroll-button"
                    disabled=true
                    aria-hidden="true"
                    aria-label="Scroll left"
                >
                    { Icon::AngleLeft }
                </button>
                <ul class="pf-c-tabs__list">
                    { for props.children.iter().enumerate().map(|(idx, c)|{
                        let current = *active == idx;
                        let active = active.clone();
                        let onselect = props.onselect.clone();
                        let onselect = Callback::from(move |_| {
                            onselect.emit(idx);
                            active.set(idx);
                        });
                        html!(<TabHeaderItem
                            label={c.props.label.clone()}
                            icon={c.props.icon.clone()}
                            {current}
                            {onselect}
                        />)
                    }) }
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

            if !props.detached {
                { for props.children.iter().enumerate().map(|(idx, mut c)| {
                    let props = Rc::make_mut(&mut c.props);
                    props.current = *active == idx;
                    c
                }) }
            }
        </>
    )
}

#[derive(Clone, Debug, PartialEq)]
pub enum TabInset {
    Inset(WithBreakpoints<Inset>),
    Page,
}

impl AsClasses for TabInset {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Page => classes.push("pf-m-page-insets"),
            Self::Inset(insets) => {
                insets.extend_classes(classes);
            }
        }
    }
}

#[derive(Clone, Debug, Properties, PartialEq)]
struct TabHeaderItemProperties {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<Icon>,

    #[prop_or_default]
    pub(crate) onselect: Callback<()>,
    #[prop_or_default]
    pub(crate) current: bool,
}

#[function_component(TabHeaderItem)]
fn tab_header_item(props: &TabHeaderItemProperties) -> Html {
    let mut classes = Classes::from("pf-c-tabs__item");

    if props.current {
        classes.push("pf-m-current");
    }
    html! (
        <li class={classes}>
            <button class="pf-c-tabs__link" onclick={props.onselect.reform(|_|())}>
                if let Some(icon) = props.icon {
                    <span class="pf-c-tabs__item-icon" aria_hidden={true.to_string()}> { icon } </span>
                }
                <span class="pf-c-tabs__item-text"> { &props.label } </span>
            </button>
        </li>
    )
}

/// Properties for [`Tab`]
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabProperties {
    pub label: String,
    #[prop_or_default]
    pub icon: Option<Icon>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub(crate) current: bool,
}

/// A tab in a [`Tabs`] component
#[function_component(Tab)]
pub fn tab(props: &TabProperties) -> Html {
    let class = Classes::from("pf-c-tab-content");

    html! (
        <section {class} hidden={!props.current}>
            { for props.children.iter() }
        </section>
    )
}
