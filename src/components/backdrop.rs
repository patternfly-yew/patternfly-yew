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
/// # use patternfly_yew::prelude::*;
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
///                     title={"Example modal"}
///                     variant={ ModalVariant::Medium }
///                     description={"A description is used when you want to provide more info about the modal than the title is able to describe."}
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
    pub children: Html,
}

#[doc(hidden)]
enum Msg {
    Open(Rc<Backdrop>),
    Close,
}

#[function_component(BackdropViewer)]
pub fn backdrop_viewer(props: &BackdropProperties) -> Html {
    // hold the state of the current backdrop
    let open = use_state::<Option<Rc<Backdrop>>, _>(|| None);

    // create the context, only once
    let ctx = {
        let open = open.clone();
        use_memo((), |()| Backdropper {
            callback: Callback::from(move |msg| match msg {
                Msg::Open(backdrop) => open.set(Some(backdrop)),
                Msg::Close => open.set(None),
            }),
        })
    };

    // when the open state changes, change the overlay
    use_effect_with(open.is_some(), |open| {
        match open {
            true => body_open(),
            false => body_close(),
        }
        body_close
    });

    // render
    html!(
        <ContextProvider<Backdropper> context={(*ctx).clone()}>
            if let Some(open) = &*open {
                <div class="pf-v6-c-backdrop">
                    { open.content.clone() }
                </div>
            }
            { props.children.clone() }
        </ContextProvider<Backdropper>>
    )
}

fn body_open() {
    if let Some(body) = document().body() {
        let classes = js_sys::Array::of1(&JsValue::from_str("pf-v6-c-backdrop__open"));
        body.class_list().add(&classes).ok();
    }
}

fn body_close() {
    if let Some(body) = document().body() {
        let classes = js_sys::Array::of1(&JsValue::from_str("pf-v6-c-backdrop__open"));
        body.class_list().remove(&classes).ok();
    }
}

/// Interact with the [`BackdropViewer`] through the [`Backdropper`].
#[hook]
pub fn use_backdrop() -> Option<Backdropper> {
    use_context::<Backdropper>()
}
