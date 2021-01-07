use crate::Title;
use yew::virtual_dom::VChild;
use yew::Properties;
use yew::{html, Children, Component, ComponentLink, Html};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub header: Children,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub footer: Children,
}

pub struct Login {
    props: Props,
}

impl Component for Login {
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

pub struct LoginMain {
    props: LoginMainProps,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for LoginMain {
    type Message = ();
    type Properties = LoginMainProps;

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
            <main class="pf-c-login__main">
                { for self.props.children.iter() }
            </main>
        }
    }
}

pub struct LoginMainHeader {
    props: LoginMainHeaderProps,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainHeaderProps {
    pub title: VChild<Title>,
    #[prop_or_default]
    pub description: String,
}

impl Component for LoginMainHeader {
    type Message = ();
    type Properties = LoginMainHeaderProps;

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
            <header class="pf-c-login__main-header">
                { self.props.title.clone() }
                <p class="pf-c-login__main-header-desc">
                    {&self.props.description}
                </p>
            </header>
        }
    }
}

pub struct LoginMainBody {
    props: LoginMainBodyProps,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainBodyProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for LoginMainBody {
    type Message = ();
    type Properties = LoginMainBodyProps;

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
            <div class="pf-c-login__main-body">
                { for self.props.children.iter() }
            </div>
        }
    }
}

pub struct LoginMainFooter {
    props: LoginMainFooterProps,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for LoginMainFooter {
    type Message = ();
    type Properties = LoginMainFooterProps;

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
            <footer>
                { for self.props.children.iter() }
            </footer>
        }
    }
}
