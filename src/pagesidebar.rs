use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(true)]
    pub open: bool,
}

#[function_component(PageSidebar)]
pub fn page_sidebar(props: &Props) -> Html {
    let mut classes = match props.open {
        true => classes!["pf-m-expanded"],
        false => classes!["pf-m-collapsed"],
    };

    classes.push("pf-c-page__sidebar");

    html! {
        <div
            aria-hidden={(!props.open).to_string()}
            class={classes}>
            <div class="pf-c-page__sidebar-body">
                { for props.children.iter() }
            </div>
        </div>
    }
}
