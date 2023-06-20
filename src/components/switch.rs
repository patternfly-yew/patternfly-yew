//! Switch control
use crate::prelude::{random_id, Icon};
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for [`Switch`]
#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProperties {
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

/// Switch component
///
/// > A **switch** toggles the state of a setting (between on and off). Switches and checkboxes can often be used interchangeably, but the switch provides a more explicit, visible representation on a setting.
///
/// See: <https://www.patternfly.org/v4/components/switch>
///
/// ## Properties
///
/// Defined by [`SwitchProperties`].
pub struct Switch {
    id: String,
    input_ref: NodeRef,
}

pub enum SwitchMsg {
    Changed,
}

impl Component for Switch {
    type Message = SwitchMsg;
    type Properties = SwitchProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let id = ctx.props().id.as_ref().cloned().unwrap_or_else(random_id);
        Self {
            id,
            input_ref: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SwitchMsg::Changed => {
                ctx.props().onchange.emit(self.current_state());
                false
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
        html! (
            <label class="pf-v5-c-switch" for={self.id.clone()}>
                <input
                    ref={self.input_ref.clone()}
                    class="pf-v5-c-switch__input"
                    type="checkbox"
                    id={self.id.clone()}
                    aria-label={ctx.props().aria_label.clone()}
                    checked={ctx.props().checked}
                    disabled={ctx.props().disabled}
                    onchange={ctx.link().callback(|_|SwitchMsg::Changed)}
                />
                <span class="pf-v5-c-switch__toggle">
                    if ctx.props().label.is_none() {
                        <span class="pf-v5-c-switch__toggle-icon">
                            { Icon::Check }
                        </span>
                    }
                </span>
                if let Some(ref label) = ctx.props().label {
                    <>
                        <span class="pf-v5-c-switch__label pf-m-on">{ label }</span>
                        <span class="pf-v5-c-switch__label pf-m-off">{ ctx.props().label_off.as_ref().unwrap_or(label) }</span>
                    </>
                }
            </label>
        )
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
