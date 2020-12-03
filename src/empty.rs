use yew::prelude::*;

use crate::{Action, Button, Icon, Size, Title, Variant};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub primary: Option<Action>,
    #[prop_or_default]
    pub secondaries: Vec<Action>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub full_height: bool,
}

#[derive(Clone, PartialEq)]
pub struct EmptyState {
    props: Props,
}

impl Component for EmptyState {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-empty-state");

        if self.props.full_height {
            classes.push("pf-m-full-height");
        }

        if let Some(size) = self.props.size {
            classes.push(size.as_class());
        }

        return html! {
            <div class=classes>
                <div class="pf-c-empty-state__content">
                    { self.render_icon() }
                    <Title size=self.title_size()>{&self.props.title}</Title>
                    <div class="pf-c-empty-state__body">
                        { for self.props.children.iter() }
                    </div>
                    { self.render_primary_action() }
                    { self.render_secondary_actions() }
                </div>
            </div>
        };
    }
}

impl EmptyState {
    fn title_size(&self) -> Size {
        match self.props.size {
            Some(Size::XLarge)
            | Some(Size::XXLarge)
            | Some(Size::XXXLarge)
            | Some(Size::XXXXLarge) => Size::XXXXLarge,
            _ => Size::Large,
        }
    }

    fn render_icon(&self) -> Html {
        match self.props.icon {
            Some(icon) => html! {icon.with_classes(Classes::from("pf-c-empty-state__icon"))},
            None => html! {},
        }
    }

    fn render_primary_action(&self) -> Html {
        match &self.props.primary {
            Some(action) => html! {
                <Button label=&action.label variant=Variant::Primary onclick=action.callback.reform(|_|{})/>
            },
            None => html! {},
        }
    }

    fn render_secondary_actions(&self) -> Html {
        if !self.props.secondaries.is_empty() {
            html! {
                <div class="pf-c-empty-state__secondary">
                    { for self.props.secondaries.iter().map(|action|{
                        html!{
                            <Button label=&action.label variant=Variant::Link onclick=action.callback.reform(|_|{})/>
                        }
                    }) }
                </div>
            }
        } else {
            html! {}
        }
    }
}
