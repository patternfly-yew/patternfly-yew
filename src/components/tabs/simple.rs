use super::TabContent;
use crate::prelude::{AsClasses, ExtendClasses, Icon, Inset, WithBreakpoints};
use crate::utils::Ouia;
use std::borrow::Cow;
use yew::html::IntoPropValue;
use yew::prelude::*;

const OUIA: Ouia = Ouia::new("Tabs");
const OUIA_BUTTON: Ouia = Ouia::new("TabsButton");
const OUIA_ITEM: Ouia = Ouia::new("TabsItem");

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

    /// OUIA Component id
    #[prop_or_else(|| OUIA.generated_id())]
    pub ouia_id: String,

    /// OUIA Component Type
    #[prop_or_else(|| OUIA.component_type())]
    pub ouia_type: String,

    /// OUIA Component Safe
    #[prop_or(true)]
    pub ouia_safe: bool,
}

/// Tabs component
///
/// > **Tabs** allow users to navigate between views within the same page or context.
///
/// See: <https://www.patternfly.org/components/tabs>
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
                data-ouia-component-id={props.ouia_id.clone()}
                data-ouia-component-type={props.ouia_type.clone()}
                data-ouia-safe={props.ouia_safe.to_string()}
            >
                <button
                    class="pf-v5-c-tabs__scroll-button"
                    disabled=true
                    aria-hidden="true"
                    aria-label="Scroll left"
                    data-ouia-component-type={OUIA_BUTTON.component_type()}
                    data-ouia-safe="true"
                    data-ouia-component-id={OUIA_BUTTON.generated_id()}
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
    pub children: Html,

    #[prop_or_default]
    pub icon: Option<Icon>,

    #[prop_or_default]
    pub onselect: Callback<T>,

    pub index: T,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// OUIA Component id
    #[prop_or_else(|| OUIA_ITEM.generated_id())]
    pub ouia_id: String,

    /// OUIA Component Type
    #[prop_or_else(|| OUIA_ITEM.component_type())]
    pub ouia_type: String,

    /// OUIA Component Safe
    #[prop_or(true)]
    pub ouia_safe: bool,
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
        <li
            {class}
            id={props.id.clone()}
            data-ouia-component-id={props.ouia_id.clone()}
            data-ouia-component-type={props.ouia_type.clone()}
            data-ouia-safe={props.ouia_safe.to_string()}
        >
            <button class="pf-v5-c-tabs__link" {onclick}>
                if let Some(icon) = props.icon {
                    <span class="pf-v5-c-tabs__item-icon" aria_hidden={true.to_string()}> { icon } </span>
                }
                <span class="pf-v5-c-tabs__item-text">
                    { props.children.clone() }
                </span>
            </button>
        </li>
    )
}

#[derive(Clone, PartialEq)]
pub enum TabTitle {
    String(Cow<'static, str>),
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

impl IntoPropValue<TabTitle> for Html {
    fn into_prop_value(self) -> TabTitle {
        TabTitle::Html(self)
    }
}

impl ToHtml for TabTitle {
    fn to_html(&self) -> Html {
        match self {
            TabTitle::String(s) => s.into(),
            TabTitle::Html(html) => html.clone(),
        }
    }

    fn into_html(self) -> Html
    where
        Self: Sized,
    {
        match self {
            TabTitle::String(s) => s.into(),
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
    pub children: Html,

    pub index: T,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub style: Option<AttrValue>,
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
        <TabContent
            hidden={!current}
            id={props.id.clone()}
            class={props.class.clone()}
            style={props.style.clone()}
        >
            { props.children.clone() }
        </TabContent>
    )
}
