use super::*;
use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuProperties {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub r#ref: NodeRef,

    #[prop_or_default]
    pub scrollable: bool,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub children: ChildrenRenderer<MenuChildVariant>,
}

#[function_component(Menu)]
pub fn menu(props: &MenuProperties) -> Html {
    let mut class = classes!("pf-v5-c-menu");

    if props.scrollable {
        class.push(classes!("pf-m-scrollable"));
    }

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    html!(
        <div
            ref={props.r#ref.clone()}
            id={props.id.clone()}
            style={&props.style}
            {class}
        >
            <div class="pf-v5-c-menu__content">
                <MenuList>{ for props.children.iter() }</MenuList>
            </div>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub(crate) struct MenuListProperties {
    pub(crate) children: ChildrenRenderer<MenuChildVariant>,
}

#[function_component(MenuList)]
pub(crate) fn menu_list(props: &MenuListProperties) -> Html {
    html!(
        <ul class="pf-v5-c-menu__list" role="menu">
            { props.children.clone() }
        </ul>
    )
}
