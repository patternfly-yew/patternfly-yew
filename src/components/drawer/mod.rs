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
    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub position: DrawerPosition,

    #[prop_or_default]
    pub inline: bool,

    #[prop_or_default]
    pub r#static: WithBreakpoints<bool>,

    #[prop_or_default]
    pub children: Children,
}

#[function_component(Drawer)]
pub fn drawer(props: &DrawerProperties) -> Html {
    let mut class = classes!("pf-v5-c-drawer");

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
                { for props.children.iter() }
            </div>
        </ContextProvider<DrawerContext>>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerContentProperties {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub panel_content: Children,
}

#[function_component(DrawerContent)]
pub fn drawer_content(props: &DrawerContentProperties) -> Html {
    let content_class = classes!("pf-v5-c-drawer__content");
    let panel_class = classes!("pf-v5-c-drawer__panel");

    let context = use_context::<DrawerContext>();
    let hidden = context.map(|context| !context.expanded).unwrap_or_default();

    html!(
        <div class={classes!("pf-v5-c-drawer__main")}>
            <div class={content_class}>
                { for props.children.iter() }
            </div>
            <div class={panel_class} {hidden}>
                { for props.panel_content.iter() }
            </div>
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerContentBodyProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DrawerContentBody)]
pub fn drawer_content_body(props: &DrawerContentBodyProperties) -> Html {
    let class = classes!("pf-v5-c-drawer__body");

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerPanelContentProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DrawerPanelContent)]
pub fn drawer_panel_content(props: &DrawerPanelContentProperties) -> Html {
    let class = classes!("pf-v5-c-drawer__body");

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerHeadProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DrawerHead)]
pub fn drawer_panel_content(props: &DrawerHeadProperties) -> Html {
    let class = classes!("pf-v5-c-drawer__head");

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerActionsProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DrawerActions)]
pub fn drawer_actions(props: &DrawerActionsProperties) -> Html {
    let class = classes!("pf-v5-c-drawer__actions");

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(PartialEq, Properties)]
pub struct DrawerCloseButtonProperties {
    pub onclick: Callback<MouseEvent>,
}

#[function_component(DrawerCloseButton)]
pub fn drawer_panel_content(props: &DrawerCloseButtonProperties) -> Html {
    let class = classes!("pf-v5-c-drawer__actions");

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
    pub children: Children,
}

#[function_component(DrawerSection)]
pub fn drawer_content_body(props: &DrawerSectionProperties) -> Html {
    let class = classes!("pf-v5-c-drawer__section");

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}
