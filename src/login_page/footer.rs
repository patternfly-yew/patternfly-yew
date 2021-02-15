use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterLinkProps {
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub children: Children,
}

pub struct LoginMainFooterLink {
    props: LoginMainFooterLinkProps,
}

impl Component for LoginMainFooterLink {
    type Message = ();
    type Properties = LoginMainFooterLinkProps;

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
        return html! {
            <a
                href=self.props.href.clone()
                target=&self.props.target
                >
                { for self.props.children.iter() }
            </a>
        };
    }
}

pub struct LoginMainFooter {
    props: LoginMainFooterProps,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub band: Children,
    #[prop_or_default]
    pub links: ChildrenWithProps<LoginMainFooterLink>,
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
        return html! {
            <footer class="pf-c-login__main-footer">
                { for self.props.children.iter() }

                { if self.props.links.len() > 0 {
                    html!{
                        <ul class="pf-c-login__main-footer-links">
                        { for self.props.links.iter().map(|item|{
                            html!{ <li class="pf-c-login__main-footer-links-item">{item}</li> }
                        }) }
                        </ul>
                    }
                } else {
                    html!{}
                }}

                { if self.props.band.len() > 0 {
                    html!{
                        <div class="pf-c-login__main-footer-band">
                        { for self.props.band.iter().map(|item|{
                            html!{ <p class="pf-c-login__main-footer-band-item">{item}</p> }
                        }) }
                        </div>
                    }
                } else {
                    html!{}
                }}

            </footer>
        };
    }
}
