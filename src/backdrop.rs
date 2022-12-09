use gloo_utils::document;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Backdrop {
    pub content: Html,
}

/// A context for displaying backdrops.
#[derive(Clone, PartialEq)]
pub struct Backdropper {
    callback: Callback<Msg>,
}

impl Backdropper {
    /// Request a backdrop from the backdrop agent.
    pub fn open(&self, backdrop: Backdrop) {
        self.callback.emit(Msg::Open(Rc::new(backdrop)));
    }

    /// Close the current backdrop.
    pub fn close(&self) {
        self.callback.emit(Msg::Close);
    }
}

// component

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

pub struct BackdropViewer {
    content: Rc<Backdrop>,
    open: bool,
    ctx: Backdropper,
}

pub enum Msg {
    Open(Rc<Backdrop>),
    Close,
}

impl Component for BackdropViewer {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let ctx = Backdropper {
            callback: ctx.link().callback(|msg| msg),
        };

        Self {
            content: Default::default(),
            open: false,
            ctx,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Open(content) => {
                self.content = content;
                self.open();
            }
            Msg::Close => {
                if self.open {
                    self.content = Default::default();
                    self.close();
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html!(
            <>
                <ContextProvider<Backdropper> context={self.ctx.clone()}>
                    if self.open {
                        <div class="pf-c-backdrop">
                            { self.content.content.clone() }
                        </div>
                    }
                    { for ctx.props().children.iter() }
                </ContextProvider<Backdropper>>
            </>
        )
    }
}

impl BackdropViewer {
    fn open(&mut self) {
        if let Some(body) = document().body() {
            let classes = js_sys::Array::of1(&JsValue::from_str("pf-c-backdrop__open"));
            body.class_list().add(&classes).ok();
        }
        self.open = true;
    }

    fn close(&mut self) {
        if let Some(body) = document().body() {
            let classes = js_sys::Array::of1(&JsValue::from_str("pf-c-backdrop__open"));
            body.class_list().remove(&classes).ok();
        }
        self.open = false;
    }
}

/// Interact with the [`BackdropViewer`] through the [`Backdropper`].
#[hook]
pub fn use_backdrop() -> Option<Backdropper> {
    use_context::<Backdropper>()
}
