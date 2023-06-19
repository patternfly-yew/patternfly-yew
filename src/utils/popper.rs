use crate::{
    integration::popperjs::{self, from_popper, Instance},
    prelude::GlobalClose,
};
use std::{fmt::Debug, marker::PhantomData};
use wasm_bindgen::{closure::Closure, JsValue};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopperProperties<T>
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

/// Support component for popper.js.
pub struct Popper<C>
where
    C: PopperContent + 'static,
    C::Properties: PartialEq + Debug,
{
    global_close: GlobalClose,
    target: NodeRef,
    popper: Option<popperjs::Instance>,
    _callback: Option<Closure<dyn Fn(&Instance)>>,

    active: bool,
    state: Option<popperjs::State>,

    _marker: PhantomData<C>,
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
    type Properties = PopperProperties<C::Properties>;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            target: NodeRef::default(),
            popper: None,
            _callback: None,
            active: false,
            state: None,
            _marker: Default::default(),
            global_close: GlobalClose::new(NodeRef::default(), ctx.link().callback(|_| Msg::Close)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                let state = Some(state);
                if self.state != state {
                    self.state = state;
                    true
                } else {
                    false
                }
            }
            Msg::Close => {
                if self.active {
                    ctx.props().onclose.emit(());
                }
                false
            }
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        let active = ctx.props().active;
        if self.active != active {
            self.active = active;
            if self.active {
                self.show(ctx).ok();
            } else {
                self.hide();
            }
            true
        } else {
            // only re-render when active
            self.active
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        self.check_update().ok();

        let onclose = ctx.link().callback(|_| Msg::Close);

        let content = <C as PopperContent>::view(
            &ctx.props().content,
            onclose,
            self.global_close.clone(),
            self.state.clone(),
        );

        let content = create_portal(content, gloo_utils::body().into());

        html! (
            <>
                <span ref={self.target.clone()}>
                    { for ctx.props().children.iter() }
                </span>
                { content }
            </>
        )
    }
}

impl<C> Popper<C>
where
    C: PopperContent,
    C::Properties: Clone + PartialEq + Debug,
{
    fn show(&mut self, ctx: &Context<Self>) -> Result<(), JsValue> {
        if self.popper.is_some() {
            return Ok(());
        }

        let target = self
            .target
            .get()
            .ok_or_else(|| JsValue::from("Missing target"))?;
        let content = self
            .global_close
            .get()
            .ok_or_else(|| JsValue::from("Missing content"))?;

        let update = ctx.link().callback(Msg::State);
        let update = Closure::wrap(Box::new(move |this: &Instance| {
            // web_sys::console::debug_2(&JsValue::from("apply: "), this);
            let msg = from_popper(this).unwrap();
            // log::info!("Msg: {:?}", msg);

            update.emit(msg);
        }) as Box<dyn Fn(&Instance)>);

        let opts = popperjs::create_default_opts(&update)?;

        //web_sys::console::debug_1(&opts);

        let popper = popperjs::create_popper(target, content, &opts);

        // web_sys::console::debug_1(&popper);
        self.popper = Some(popper);
        self._callback = Some(update);

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
