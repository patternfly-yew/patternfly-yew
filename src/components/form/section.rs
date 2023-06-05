use uuid::Uuid;
use yew::prelude::*;

/// Properties for [`FormSection`]
#[derive(Clone, PartialEq, Properties)]
pub struct FormSectionProperties {
    pub title: Option<String>,
    pub children: Children,
}

/// A group of fields on a [`Form`](crate::prelude::Form)
#[function_component(FormSection)]
pub fn section(props: &FormSectionProperties) -> Html {
    let id = use_state(|| match props.title.is_some() {
        true => Some(Uuid::new_v4().to_string()),
        false => None,
    });

    html! (
        <section class="pf-v5-c-form__section" role="group" aria-labelledby={(*id).clone()}>

            if let Some(title) = &props.title {
                <div id={(*id).clone()} class="pf-v5-c-form__section-title" aria-hidden="true">
                    { title }
                </div>
            }

            { for props.children.iter() }
        </section>
    )
}
