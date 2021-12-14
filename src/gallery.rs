use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
}

#[function_component(Gallery)]
pub fn gallery(props: &Props) -> Html {
    let mut classes = Classes::from("pf-l-gallery");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    html! {
        <div class={classes}>
        { for props.children.iter().map(|child|{
            html!{
                <div class="pf-l-gallery__item">
                    { child }
                </div>
            }
        }) }
        </div>
    }
}
