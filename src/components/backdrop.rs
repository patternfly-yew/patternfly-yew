//! Backdrop visual
use gloo_utils::document;
use std::rc::Rc;
use wasm_bindgen::JsValue;
use yew::prelude::*;

/// Backdrop overlay the main content and show some new content, until it gets closed.
///
/// New content can be sent to the backdrop viewer using the [`Backdropper::open`] call. It can be
/// closed using the [`Backdropper::close`] call.
///
/// ## Contexts
///
/// The [`BackdropViewer`] must be wrapped by all contexts which the backdrop content might use,
/// as the content is injected as a child into the backdrop element. So if you can to send toasts
/// from a modal dialog, the [`ToastViewer`](crate::prelude::ToastViewer) must be wrapping the
/// [`BackdropViewer`].
///
/// ## Example
///
/// ```
/// # use yew::prelude::*;
/// # use patternfly_yew::*;
/// #[function_component(App)]
/// fn app() -> Html {
///   html! {
///     <>
///       <BackdropViewer>
///         <View/>
///       </BackdropViewer>
///     </>
///   }
/// }
/// #[function_component(View)]
/// fn view() -> Html {
///   let backdropper = use_backdrop().expect("Must be nested under a BackdropViewer component");
///   html!{
///     <div>
///       <button onclick={move |_| backdropper.open(Backdrop::new(
///         html! {
///             <Bullseye>
///                 <Modal
///                     title = {"Example modal"}
///                     variant = { ModalVariant::Medium }
///                     description = {"A description is used when you want to provide more info about the modal than the title is able to describe."}
///                 >
///                     <p>{"The modal body can contain text, a form, any nested html will work."}</p>
///                 </Modal>
///             </Bullseye>
///         }))
///       }>
///         { "Click me" }  
///       </button>
///     </div>
///   }
/// }
/// ```
#[derive(Clone, Debug)]
pub struct Backdrop {
    pub content: Html,
}

impl Backdrop {
    pub fn new(content: Html) -> Self {
        Self { content }
    }
}

impl Default for Backdrop {
    fn default() -> Self {
        Self { content: html!() }
    }
}

impl From<Html> for Backdrop {
    fn from(content: Html) -> Self {
        Self { content }
    }
}

/// A context for displaying backdrops.
#[derive(Clone, PartialEq)]
pub struct Backdropper {
    callback: Callback<Msg>,
}

impl Backdropper {
    /// Request a backdrop from the backdrop agent.
    pub fn open<B>(&self, backdrop: B)
    where
        B: Into<Backdrop>,
    {
        self.callback.emit(Msg::Open(Rc::new(backdrop.into())));
    }

    /// Close the current backdrop.
    pub fn close(&self) {
        self.callback.emit(Msg::Close);
    }
}

/// Properties for [``BackdropViewer]
#[derive(Clone, PartialEq, Properties)]
pub struct BackdropProperties {
    pub children: Children,
}

/// The component showing backdrop content
pub struct BackdropViewer {
    content: Rc<Backdrop>,
    open: bool,
    ctx: Backdropper,
}

#[doc(hidden)]
pub enum Msg {
    Open(Rc<Backdrop>),
    Close,
}

impl Component for BackdropViewer {
    type Message = Msg;
    type Properties = BackdropProperties;

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
