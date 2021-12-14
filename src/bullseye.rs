use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    /// Allows to disable wrapping children in the "item" div.
    ///
    /// According to the PatternFly documentation, this shouldn't make a difference. In practice,
    /// sometimes it does. Like when hosting a modal about dialog.
    pub plain: bool,
}

/// Bullseye layout.
///
/// https://www.patternfly.org/v4/layouts/bullseye
#[function_component(Bullseye)]
pub fn bullseye(props: &Props) -> Html {
    html! {
        <div class="pf-l-bullseye">
            { for props.children.iter().map(|c|{
                if props.plain {
                    // according to the PatternFly documentation wrapping element with the item
                    // shouldn't make a difference. In practice, sometimes it does.
                    c
                } else {html!{
                    <div class="pf-l-bullseye__item">{c}</div>
                }}
            }) }
        </div>
    }
}
