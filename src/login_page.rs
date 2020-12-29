use yew::Properties;
use yew::{html, Children, Component, ComponentLink, Html};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub header: Children,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub footer: Children,
}

pub struct LoginPage {
    props: Props,
    link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for LoginPage {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {}

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
            <div class="pf-c-login">
                <div class="pf-c-login__container">
                    {
                        if !self.props.header.is_empty() {
                            html!{ <header class="pf-c-login__header">{for self.props.header.iter()}</header> }
                        } else {
                            html!{}
                        }
                    }
                    <main class="pf-c-login__main">
                        { for self.props.children.iter() }
                    </main>
                    {
                        if !self.props.footer.is_empty() {
                            html!{ <footer class="pf-c-login__footer">{for self.props.footer.iter()}</footer> }
                        } else {
                            html!{}
                        }
                    }
                </div>
            </div>
        }
    }
}

impl LoginPage {}
