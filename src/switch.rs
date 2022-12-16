use crate::{random_id, Icon};
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub label_off: Option<String>,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub aria_label: String,
}

pub struct Switch {
    id: String,
    input_ref: NodeRef,
}

pub enum Msg {
    Changed,
}

impl Component for Switch {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx
            .props()
            .id
            .as_ref()
            .cloned()
            .unwrap_or_else(|| random_id());
        Self {
            id,
            input_ref: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Changed => {
                ctx.props().onchange.emit(self.current_state());
                return false;
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        self.id = ctx
            .props()
            .id
            .as_ref()
            .cloned()
            .unwrap_or_else(|| self.id.clone());
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <label class="pf-c-switch" for={self.id.clone()}>
                <input
                    ref={self.input_ref.clone()}
                    class="pf-c-switch__input"
                    type="checkbox"
                    id={self.id.clone()}
                    aria-label={ctx.props().aria_label.clone()}
                    checked={ctx.props().checked}
                    disabled={ctx.props().disabled}
                    onchange={ctx.link().callback(|_|Msg::Changed)}
                    />
                <span class="pf-c-switch__toggle">
                    if ctx.props().label.is_none() {
                        <span class="pf-c-switch__toggle-icon">
                            { Icon::Check }
                        </span>
                    }
                </span>
                if let Some(ref label) = ctx.props().label {
                    <>
                    <span class="pf-c-switch__label pf-m-on">{ label }</span>
                    <span class="pf-c-switch__label pf-m-off">{ ctx.props().label_off.as_ref().unwrap_or_else(||label) }</span>
                    </>
                }
            </label>
        }
    }
}

impl Switch {
    fn current_state(&self) -> bool {
        self.input_ref
            .cast::<HtmlInputElement>()
            .map(|input| input.checked())
            .unwrap_or_default()
    }
}
