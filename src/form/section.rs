use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub title: Option<String>,
    pub children: Children,
}

#[function_component(FormSection)]
pub fn badge(props: &Props) -> Html {
    let id = use_state(|| props.title.as_ref().map(|_| Uuid::new_v4().to_string()));

    html! (
        <section class="pf-c-form__section" role="group" aria-labelledby={(*id).clone()}>

            if let Some(title) = &props.title {
                <div id={(*id).clone()} class="pf-c-form__section-title" aria-hidden="true">
                    { title }
                </div>
            }

            { for props.children.iter() }
        </section>
    )
}
