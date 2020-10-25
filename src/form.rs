use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

pub struct Form {
    props: Props,
}

impl Component for Form {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <form novalidate=true class="pf-c-form">
                { for self.props.children.iter().map(|child|{
                        child
                }) }
            </form>
        }
    }
}

// form group

#[derive(Clone, PartialEq, Properties)]
pub struct FormGroupProps {
    pub children: Children,
    pub label: String,
    #[prop_or_default]
    pub required: bool,
    #[prop_or_default]
    pub helper_text: String,
}

pub struct FormGroup {
    props: FormGroupProps,
}

impl Component for FormGroup {
    type Message = ();
    type Properties = FormGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let classes = Classes::from("pf-c-form__group");

        html! {
            <div class=classes>
                <div class="pf-c-form__group-label">
                    <div class="pf-c-form__label">
                        <span class="pf-c-form__label-text">{&self.props.label}</span>

                        {if self.props.required {
                            html!{
                                <span class="pf-c-form__label-required" aria-hidden="true">{"*"}</span>
                            }
                        } else {
                            html!{}
                        }}

                    </div>
                </div>
                <div class="pf-c-form__group-control">
                    { for self.props.children.iter() }
                    { if !self.props.helper_text.is_empty() {html!{
                        <p class="pf-c-form__helper-text" aria-live="polite">{ &self.props.helper_text }</p>
                    }} else {html!{}}}
                </div>
            </div>
        }
    }
}
