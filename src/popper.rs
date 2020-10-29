use crate::integration::popperjs;

use std::fmt::Debug;
use wasm_bindgen::JsValue;
use yew::prelude::*;

// popper

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props<T>
where
    T: Clone + PartialEq + Debug,
{
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub active: bool,

    pub content: T,

    /// Close callback that will be emitted when the popper's component will emit the onclose callback.
    #[prop_or_default]
    pub onclose: Callback<()>,
}

pub struct Popper<C>
where
    C: PopperContent + 'static,
    C::Properties: PartialEq + Debug,
{
    props: Props<C::Properties>,
    link: ComponentLink<Self>,

    target: NodeRef,
    content: NodeRef,
    popper: Option<popperjs::Instance>,

    state: Option<popperjs::State>,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Close,
    State(popperjs::State),
}

impl<C> Component for Popper<C>
where
    C: PopperContent + 'static,
    C::Properties: Clone + PartialEq + Debug,
{
    type Message = Msg;
    type Properties = Props<C::Properties>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            target: NodeRef::default(),
            content: NodeRef::default(),
            popper: None,
            state: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::State(state) => {
                let state = Some(state);
                let mut changed = false;
                if self.state != state {
                    self.state = state;
                    changed = true;
                }
                changed
            }
            Msg::Close => {
                self.props.onclose.emit(());
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props.active != props.active {
            if props.active {
                self.show().unwrap();
            } else {
                self.hide();
            }
        }

        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        self.check_update().ok();

        let onclose = self.link.callback(|_| Msg::Close);

        let content = <C as PopperContent>::view(
            &self.props.content,
            onclose,
            self.content.clone(),
            self.state.clone(),
        );

        return html! {
            <>
                <span ref=self.target.clone()>
                    { for self.props.children.iter() }
                </span>
                { content }
            </>
        };
    }

    fn destroy(&mut self) {}
}

impl<C> Popper<C>
where
    C: PopperContent,
    C::Properties: Clone + PartialEq + Debug,
{
    fn show(&mut self) -> Result<(), JsValue> {
        if self.popper.is_some() {
            return Ok(());
        }

        let target = self
            .target
            .get()
            .ok_or_else(|| JsValue::from("Missing target"))?;
        let content = self
            .content
            .get()
            .ok_or_else(|| JsValue::from("Missing content"))?;

        let update = self.link.callback(|state| Msg::State(state));
        let opts = popperjs::create_default_opts(update)?;

        //web_sys::console::debug_1(&opts);

        let popper = popperjs::create_popper(target, content, &opts);

        // web_sys::console::debug_1(&popper);
        self.popper = Some(popper);

        Ok(())
    }

    fn hide(&mut self) {
        self.destroy();
        self.state = None;
    }

    fn check_update(&self) -> Result<(), JsValue> {
        if let Some(popper) = &self.popper {
            popper.update();
        }
        Ok(())
    }

    fn destroy(&mut self) {
        if let Some(popper) = self.popper.take() {
            popper.destroy();
        }
    }
}

pub trait PopperContent: Component {
    fn view(
        props: &Self::Properties,
        onclose: Callback<()>,
        r#ref: NodeRef,
        state: Option<popperjs::State>,
    ) -> Html;
}
