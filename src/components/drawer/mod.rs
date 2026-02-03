use crate::prelude::*;
use yew::prelude::*;

#[derive(PartialEq, Eq, Copy, Clone, Default)]
pub enum DrawerPosition {
    Left,
    #[default]
    Right,
    Bottom,
}

impl AsClasses for DrawerPosition {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Left => {
                classes.push(classes!("pf-m-panel-left"));
            }
            Self::Right => {}
            Self::Bottom => {
                classes.push(classes!("pf-m-panel-bottom"));
            }
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct DrawerContext {
    pub expanded: bool,
}

#[derive(PartialEq, Properties)]
pub struct DrawerProperties {
    /// The expanded state.
    #[prop_or_default]
    pub expanded: bool,

    /// Position of the drawer panel when opened.
    #[prop_or_default]
    pub position: DrawerPosition,

    /// Whether the drawer panel overlaps or replaces the content.
    #[prop_or_default]
    pub inline: bool,

    #[prop_or_default]
    pub r#static: WithBreakpoints<bool>,

    #[prop_or_default]
    pub children: Html,
}

/// Drawer component
///
/// > A **drawer** is a sliding panel that enters from the right edge of the viewport. It can be configured to either overlay content on a page or create a sidebar by pushing that content to the left.
///
/// See: <https://www.patternfly.org/components/drawer>
///
/// ## Properties
///
/// Defined by [`DrawerProperties`].
///
/// ## Children
///
/// The drawer requires a structure of other drawer elements, which isn't enforced through types.
/// See the example, the quickstart project, and the PatternFly documentation for guidance.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///
///   let expanded = use_state_eq(|| false);
///   let onclick = use_callback(expanded.clone(), |_, expanded| expanded.set(!**expanded));
///   let onclose = use_callback( expanded.clone(), |_, expanded| expanded.set(false));
///
///   let panel_content = html!(
///     <DrawerPanelContent>
///       <DrawerHead>
///         <span>
///           {"drawer-panel"}
///         </span>
///         <DrawerActions>
///           <DrawerCloseButton onclick={onclose} />
///         </DrawerActions>
///       </DrawerHead>
///     </DrawerPanelContent>
///   );
///
///   html!(
///     <>
///       <Button variant={ButtonVariant::Primary} {onclick}>{"Toggle drawer"}</Button>
///       <Drawer expanded={*expanded}>
///         <DrawerContent {panel_content}>
///           <DrawerContentBody>
///             { "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Phasellus pretium est a porttitor vehicula. Quisque vel commodo urna. Morbi mattis rutrum ante, id vehicula ex accumsan ut. Morbi viverra, eros vel porttitor facilisis, eros purus aliquet erat,nec lobortis felis elit pulvinar sem. Vivamus vulputate, risus eget commodo eleifend, eros nibh porta quam, vitae lacinia leo libero at magna. Maecenas aliquam sagittis orci, et posuere nisi ultrices sit amet. Aliquam ex odio, malesuada sed posuere quis, pellentesque at mauris. Phasellus venenatis massa ex, eget pulvinar libero auctor pretium. Aliquam erat volutpat. Duis euismod justo in quam ullamcorper, in commodo massa vulputate." }
///           </DrawerContentBody>
///         </DrawerContent>
///       </Drawer>
///     </>
///   )
/// }
/// ```
///
/// For more examples, see the PatternFly Yew Quickstart project.
#[function_component(Drawer)]
pub fn drawer(props: &DrawerProperties) -> Html {
    let mut class = classes!("pf-v6-c-drawer");

    if props.expanded {
        class.extend(classes!("pf-m-expanded"));
    }

    class.extend_from(&props.position);

    if props.inline {
        class.extend(classes!("pf-m-inline"));
    }

    class.extend_from(&props.r#static.mapped(|f| f.then(|| "static".to_string())));

    let context = DrawerContext {
        expanded: props.expanded,
    };

    html!(
        <ContextProvider<DrawerContext> {context}>
            <div {class}>
                { props.children.clone() }
            </div>
        </ContextProvider<DrawerContext>>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerContentProperties {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub panel_content: Html,
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProperties) -> Html {
    let content_class = classes!("pf-v6-c-drawer__content");
    let panel_class = classes!("pf-v6-c-drawer__panel");

    let context = use_context::<DrawerContext>();
    let hidden = context.map(|context| !context.expanded).unwrap_or_default();

    html!(
        <div class={classes!("pf-v6-c-drawer__main")}>
            <div class={content_class}>
                { props.children.clone() }
            </div>
            <div class={panel_class} {hidden}>
                { props.panel_content.clone() }
            </div>
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerContentBodyProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(DrawerContentBody)]
pub fn drawer_content_body(props: &DrawerContentBodyProperties) -> Html {
    let class = classes!("pf-v6-c-drawer__body");

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerPanelContentProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(DrawerPanelContent)]
pub fn drawer_panel_content(props: &DrawerPanelContentProperties) -> Html {
    let class = classes!("pf-v6-c-drawer__body");

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerHeadProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(DrawerHead)]
pub fn drawer_panel_content(props: &DrawerHeadProperties) -> Html {
    let class = classes!("pf-v6-c-drawer__head");

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerActionsProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(DrawerActions)]
pub fn drawer_actions(props: &DrawerActionsProperties) -> Html {
    let class = classes!("pf-v6-c-drawer__actions");

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerCloseButtonProperties {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(DrawerCloseButton)]
pub fn drawer_panel_content(props: &DrawerCloseButtonProperties) -> Html {
    let class = classes!("pf-v6-c-drawer__actions");

    html!(
        <div {class}>
            <Button
                onclick={props.onclick.clone()}
                variant={ButtonVariant::Plain}
                icon={Icon::Times}
                r#type={ButtonType::Button}
                aria_label="Close drawer panel"
            />
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerSectionProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(DrawerSection)]
pub fn drawer_content_body(props: &DrawerSectionProperties) -> Html {
    let class = classes!("pf-v6-c-drawer__section");

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}
