use yew::prelude::*;

use crate::{Action, Icon, Size, Title, button::{Button, Variant}};

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
pub struct EmptyState {}

impl Component for EmptyState {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-empty-state");

        if ctx.props().full_height {
            classes.push("pf-m-full-height");
        }

        if let Some(size) = ctx.props().size {
            classes.push(size.as_class());
        }

        return html! {
            <div class={classes}>
                <div class="pf-c-empty-state__content">
                    { self.render_icon(ctx) }
                    <Title size={ self.title_size(ctx) }>{&ctx.props().title}</Title>
                    <div class="pf-c-empty-state__body">
                        { for ctx.props().children.iter() }
                    </div>
                    { self.render_primary_action(ctx) }
                    { self.render_secondary_actions(ctx) }
                </div>
            </div>
        };
    }
}

impl EmptyState {
    fn title_size(&self, ctx: &Context<Self>) -> Size {
        match ctx.props().size {
            Some(Size::XLarge)
            | Some(Size::XXLarge)
            | Some(Size::XXXLarge)
            | Some(Size::XXXXLarge) => Size::XXXXLarge,
            _ => Size::Large,
        }
    }

    fn render_icon(&self, ctx: &Context<Self>) -> Html {
        match ctx.props().icon {
            Some(icon) => html! {icon.with_classes(Classes::from("pf-c-empty-state__icon"))},
            None => html! {},
        }
    }

    fn render_primary_action(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().primary {
            Some(action) => html! {
                <Button label={action.label.clone()} variant={Variant::Primary} onclick={action.callback.reform(|_|{})}/>
            },
            None => html! {},
        }
    }

    fn render_secondary_actions(&self, ctx: &Context<Self>) -> Html {
        if !ctx.props().secondaries.is_empty() {
            html! {
                <div class="pf-c-empty-state__secondary">
                    { for ctx.props().secondaries.iter().map(|action|{
                        html!{
                            <Button label={action.label.clone()} variant={Variant::Link} onclick={action.callback.reform(|_|{})}/>
                        }
                    }) }
                </div>
            }
        } else {
            html! {}
        }
    }
}
