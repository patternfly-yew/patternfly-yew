use crate::{random_id, Icon};
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
    pub on_change: Callback<ChangeData>,

    #[prop_or_default]
    pub aria_label: String,
}

pub struct Switch {
    props: Props,

    id: String,
}

impl Component for Switch {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        let id = props.id.as_ref().cloned().unwrap_or_else(|| random_id());
        Self { props, id }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            let id = props
                .id
                .as_ref()
                .cloned()
                .unwrap_or_else(|| self.id.clone());
            self.props = props;
            self.id = id;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <label class="pf-c-switch" for=self.id>
                <input
                    class="pf-c-switch__input"
                    type="checkbox"
                    id=self.id
                    aria-label=self.props.aria_label
                    checked=self.props.checked
                    disabled=self.props.disabled
                    onchange=self.props.on_change.clone()
                    />
                <span class="pf-c-switch__toggle">
                    { if self.props.label.is_none() { html!{
                    <span class="pf-c-switch__toggle-icon">
                        { Icon::Check }
                    </span>
                    }} else { html!{}} }
                </span>
                { if let Some(ref label) = self.props.label {html!{
                    <>
                    <span class="pf-c-switch__label pf-m-on">{label}</span>
                    <span class="pf-c-switch__label pf-m-off">{self.props.label_off.as_ref().unwrap_or_else(||label)}</span>
                    </>
                }} else {html!{}}}
            </label>
        }
    }
}
