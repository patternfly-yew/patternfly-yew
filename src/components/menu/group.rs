use super::MenuList;
use crate::prelude::MenuChildVariant;
use yew::{html::ChildrenRenderer, prelude::*};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuGroupProperties {
    #[prop_or_default]
    pub title: Option<String>,
    #[prop_or_default]
    pub children: ChildrenRenderer<MenuChildVariant>,
}

#[function_component(MenuGroup)]
pub fn menu_group(props: &MenuGroupProperties) -> Html {
    html!(
        <section class="pf-v6-c-menu__group">
            if let Some(title) = &props.title {
                <h1 class="pf-v6-c-menu__group-title">{ title }</h1>
            }
            <MenuList>{ props.children.clone() }</MenuList>
        </section>
    )
}
