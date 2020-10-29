use crate::{Button, Icon, Orientation, Popper, PopperContent, Variant};

use yew::prelude::*;

use crate::integration::popperjs;

// tooltip

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopoverProps {
    /// The target, rendered by the component, to which the popover will be aligned to.
    #[prop_or_default]
    pub target: Html,
    /// Flag if the popover is visible or not.
    #[prop_or_default]
    pub active: bool,

    /// The header content of the popover.
    #[prop_or_default]
    pub header: Html,
    /// The content which will be show in the popover.
    pub children: Children,
    /// The footer content of the popover.
    #[prop_or_default]
    pub footer: Html,

    /// Binds the onclick handler of the target to toggle visibility.
    #[prop_or_default]
    pub toggle_by_onclick: bool,
}

pub struct Popover {
    props: PopoverProps,
    link: ComponentLink<Self>,
    node: NodeRef,
}

#[derive(Clone, Debug)]
pub enum PopoverMsg {
    Toggle,
    Close,
}

impl Component for Popover {
    type Message = PopoverMsg;
    type Properties = PopoverProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            node: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PopoverMsg::Toggle => {
                self.props.active = !self.props.active;
                true
            }
            PopoverMsg::Close => {
                if self.props.active {
                    self.props.active = false;
                    true
                } else {
                    false
                }
            }
        }
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
        let (onclick, onclose) = match self.props.toggle_by_onclick {
            true => (
                self.link.callback(|_| PopoverMsg::Toggle),
                self.link.callback(|_| PopoverMsg::Close),
            ),
            false => Default::default(),
        };

        return html! {
            <>
                <Popper<Popover>
                    active=self.props.active
                    content=self.props.clone()
                    onclose=onclose
                    >
                    <span onclick=onclick ref=self.node.clone()>
                        { self.props.target.clone() }
                    </span>
                </Popper<Popover>>
            </>
        };
    }
}

impl PopperContent for Popover {
    fn view(
        props: &PopoverProps,
        onclose: Callback<()>,
        r#ref: NodeRef,
        state: Option<popperjs::State>,
    ) -> Html {
        let styles: &str = match &state {
            Some(state) => &state.styles,
            None => "display: none;",
        };

        let orientation = state
            .as_ref()
            .map(|s| s.orientation)
            .unwrap_or(Orientation::Bottom);

        html! {
            <PopoverPopup
                ref=r#ref
                styles=styles
                orientation=orientation
                header=&props.header
                footer=&props.footer
                children=&props.children
                onclose=onclose
            />
        }
    }
}

// popover popup

#[derive(Clone, PartialEq, Properties)]
pub struct PopoverPopupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub header: Html,
    #[prop_or_default]
    pub footer: Option<Html>,
    pub orientation: Orientation,
    #[prop_or_default]
    pub hidden: bool,
    #[prop_or_default]
    pub styles: String,

    /// called when the close button is clicked
    #[prop_or_default]
    pub onclose: Callback<()>,
}

#[derive(Clone)]
pub struct PopoverPopup {
    props: PopoverPopupProps,
    link: ComponentLink<Self>,
}

#[derive(Copy, Debug, Clone)]
pub enum PopoverPopupMsg {
    Close,
}

impl Component for PopoverPopup {
    type Message = PopoverPopupMsg;
    type Properties = PopoverPopupProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PopoverPopupMsg::Close => {
                self.props.onclose.emit(());
                false
            }
        }
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
        let mut classes = Classes::from("pf-c-popover");

        classes = classes.extend(self.props.orientation.as_classes());

        let style = if self.props.hidden {
            "display: none;"
        } else {
            &self.props.styles
        };

        let onclose = self.link.callback(|_| PopoverPopupMsg::Close);

        return html! {
            <div style=style class=classes role="dialog" aria-model="true">
                <div class="pf-c-popover__arrow"></div>
                <div class="pf-c-popover__content">
                    { self.props.header.clone() }

                    <Button
                        variant=Variant::Plain
                        icon=Icon::Times
                        aria_label="Close"
                        onclick=onclose
                    />

                    <div class="pf-c-popover__body">
                        { for self.props.children.iter() }
                    </div>

                    { if let Some(footer) = &self.props.footer {
                        html!{
                            <footer class="pf-c-popover__footer">
                                { footer.clone() }
                            </footer>
                        }
                    } else {
                        html!{}
                    }}

                </div>
            </div>
        };
    }
}
