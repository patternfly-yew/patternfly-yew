use yew::prelude::*;

/// Properties for [`PageSidebar`]
#[derive(Clone, PartialEq, Properties)]
pub struct PageSidebarProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub open: bool,
}

/// The sidebar component of a [`Page`](crate::prelude::Page).
///
/// ## Properties
///
/// Defined by [`PageSidebarProperties`].
#[function_component(PageSidebar)]
pub fn page_sidebar(props: &PageSidebarProperties) -> Html {
    let mut classes = match props.open {
        true => classes!["pf-m-expanded"],
        false => classes!["pf-m-collapsed"],
    };

    classes.push("pf-v5-c-page__sidebar");

    html! (
        <div
            aria-hidden={(!props.open).to_string()}
            class={classes}>
            <div class="pf-v5-c-page__sidebar-body">
                { for props.children.iter() }
            </div>
        </div>
    )
}
