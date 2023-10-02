use super::TabContent;
use crate::prelude::{AsClasses, ExtendClasses, Icon, Inset, WithBreakpoints};
use std::borrow::Cow;
use yew::html::IntoPropValue;
use yew::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub struct TabsContext<T>
where
    T: PartialEq + Eq + Clone + 'static,
{
    pub selected: T,
}

/// Properties for [`Tabs`]
#[derive(Clone, Debug, Properties, PartialEq)]
pub struct TabsProperties<T>
where
    T: PartialEq + Eq + Clone + 'static,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<Tab<T>>,

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
    pub onselect: Callback<T>,

    /// Set the current active tab, overrides the internal state.
    pub selected: T,
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
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   #[derive(Clone, Copy, PartialEq, Eq)]
///   enum MyIndex {
///     Foo,
///     Bar,
///   }
///
///   let selected = use_state_eq(|| MyIndex::Foo);
///   let onselect = use_callback(selected.clone(), |index, selected| selected.set(index));
///
///   html!(
///     <Tabs<MyIndex> selected={*selected} {onselect}>
///       <Tab<MyIndex> index={MyIndex::Foo} title="Foo">
///         {"Foo"}
///       </Tab<MyIndex>>
///       <Tab<MyIndex> index={MyIndex::Bar} title="Bar">
///         {"Bar"}
///       </Tab<MyIndex>>
///     </Tabs<MyIndex>>
///   )
/// }
/// ```
///
/// For more examples, see the PatternFly Yew Quickstart project.
#[function_component(Tabs)]
pub fn tabs<T>(props: &TabsProperties<T>) -> Html
where
    T: PartialEq + Eq + Clone + 'static,
{
    let mut class = classes!("pf-v5-c-tabs");

    if props.r#box {
        class.push(classes!("pf-m-box"));
    }

    if props.vertical {
        class.push(classes!("pf-m-vertical"));
    }

    if props.filled {
        class.push(classes!("pf-m-fill"));
    }

    class.extend_from(&props.inset);

    let context = TabsContext {
        selected: props.selected.clone(),
    };

    html! (
        <ContextProvider<TabsContext<T>> {context}>
            <div
                {class}
                id={props.id.clone()}
            >
                <button
                    class="pf-v5-c-tabs__scroll-button"
                    disabled=true
                    aria-hidden="true"
                    aria-label="Scroll left"
                >
                    { Icon::AngleLeft }
                </button>
                <ul class="pf-v5-c-tabs__list">
                    { for props.children.iter().map(|c| {
                        let onselect = props.onselect.clone();
                        html!(
                            <TabHeaderItem<T>
                                icon={c.props.icon}
                                index={c.props.index.clone()}
                                {onselect}
                            >
                                { c.props.title.to_html() }
                            </TabHeaderItem<T>>
                        )
                    }) }
                </ul>
                <button
                    class="pf-v5-c-tabs__scroll-button"
                    disabled=true
                    aria-hidden="true"
                    aria-label="Scroll right"
                >
                    { Icon::AngleRight }
                </button>
            </div>

            if !props.detached {
                { for props.children.iter() }
            }
        </ContextProvider<TabsContext<T>>>
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
struct TabHeaderItemProperties<T>
where
    T: PartialEq + Eq + Clone + 'static,
{
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub icon: Option<Icon>,

    #[prop_or_default]
    pub onselect: Callback<T>,

    pub index: T,
}

#[function_component(TabHeaderItem)]
fn tab_header_item<T>(props: &TabHeaderItemProperties<T>) -> Html
where
    T: PartialEq + Eq + Clone + 'static,
{
    let context = use_context::<TabsContext<T>>();
    let current = context
        .map(|context| context.selected == props.index)
        .unwrap_or_default();

    let mut class = Classes::from("pf-v5-c-tabs__item");

    if current {
        class.push("pf-m-current");
    }

    let onclick = use_callback(
        (props.index.clone(), props.onselect.clone()),
        |_, (index, onselect)| {
            onselect.emit(index.clone());
        },
    );

    html! (
        <li {class}>
            <button class="pf-v5-c-tabs__link" {onclick}>
                if let Some(icon) = props.icon {
                    <span class="pf-v5-c-tabs__item-icon" aria_hidden={true.to_string()}> { icon } </span>
                }
                <span class="pf-v5-c-tabs__item-text">
                    { for props.children.iter() }
                </span>
            </button>
        </li>
    )
}

#[derive(Clone, PartialEq)]
pub enum TabTitle {
    String(Cow<'static, str>),
    Children(Children),
    Html(Html),
}

impl IntoPropValue<TabTitle> for String {
    fn into_prop_value(self) -> TabTitle {
        TabTitle::String(self.into())
    }
}

impl IntoPropValue<TabTitle> for &'static str {
    fn into_prop_value(self) -> TabTitle {
        TabTitle::String(self.into())
    }
}

impl IntoPropValue<TabTitle> for Children {
    fn into_prop_value(self) -> TabTitle {
        TabTitle::Children(self)
    }
}

impl IntoPropValue<TabTitle> for Html {
    fn into_prop_value(self) -> TabTitle {
        TabTitle::Html(self)
    }
}

impl ToHtml for TabTitle {
    fn to_html(&self) -> Html {
        match self {
            TabTitle::String(s) => s.into(),
            TabTitle::Children(children) => children.iter().collect(),
            TabTitle::Html(html) => html.clone(),
        }
    }

    fn into_html(self) -> Html
    where
        Self: Sized,
    {
        match self {
            TabTitle::String(s) => s.into(),
            TabTitle::Children(children) => children.into_iter().collect(),
            TabTitle::Html(html) => html,
        }
    }
}

/// Properties for [`Tab`]
#[derive(Properties, PartialEq)]
pub struct TabProperties<T>
where
    T: PartialEq + Eq + Clone + 'static,
{
    pub title: TabTitle,

    #[prop_or_default]
    pub icon: Option<Icon>,

    #[prop_or_default]
    pub children: Children,

    pub index: T,
}

/// A tab in a [`Tabs`] component
#[function_component(Tab)]
pub fn tab<T>(props: &TabProperties<T>) -> Html
where
    T: PartialEq + Eq + Clone + 'static,
{
    let context = use_context::<TabsContext<T>>();
    let current = context
        .map(|context| context.selected == props.index)
        .unwrap_or_default();

    html! (
        <TabContent hidden={!current}>
            { for props.children.iter() }
        </TabContent>
    )
}
