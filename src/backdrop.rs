use std::collections::HashSet;
use wasm_bindgen::JsValue;
use yew::agent::*;
use yew::prelude::*;
use yew::utils::window;

#[derive(Clone, Debug)]
pub struct Backdrop {
    pub content: Html,
}

#[doc(hidden)]
#[derive(Debug)]
pub enum BackdropRequest {
    Open(Backdrop),
    Close,
}

#[doc(hidden)]
pub enum BackdropAction {
    Open(Backdrop),
    Close,
}

/// An agent for displaying toasts.
pub struct Backdropper {
    link: AgentLink<Self>,
    /// The backdrop viewer.
    ///
    /// While we can handle more than one, we will only send backdrops to one viewer. Registering
    /// more than one viewer will produce unexpected results.
    viewer: HashSet<HandlerId>,
}

impl Agent for Backdropper {
    type Reach = Context<Self>;
    type Message = ();
    type Input = BackdropRequest;
    type Output = BackdropAction;

    fn create(link: AgentLink<Self>) -> Self {
        Self {
            link,
            viewer: HashSet::new(),
        }
    }

    fn update(&mut self, _: Self::Message) {}

    fn connected(&mut self, id: HandlerId) {
        if id.is_respondable() {
            self.viewer.insert(id);
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _: HandlerId) {
        match msg {
            BackdropRequest::Open(backdrop) => {
                self.notify_backdrop(BackdropAction::Open(backdrop));
            }
            BackdropRequest::Close => {
                self.notify_backdrop(BackdropAction::Close);
            }
        }
    }

    fn disconnected(&mut self, id: HandlerId) {
        if id.is_respondable() {
            self.viewer.remove(&id);
        }
    }
}

impl Backdropper {
    fn notify_backdrop(&self, msg: BackdropAction) {
        let viewer = self.viewer.iter().next();
        if let Some(viewer) = viewer {
            self.link.respond(*viewer, msg);
        } else {
            window()
                .alert_with_message(&format!(
                    "Dropped backdrop. No backdrop component registered."
                ))
                .ok();
        }
    }
}

/// Client to the backdrop agent which can be used to request backdrops.
pub struct BackdropDispatcher(Dispatcher<Backdropper>);

impl BackdropDispatcher {
    pub fn new() -> Self {
        Self(Backdropper::dispatcher())
    }

    /// Request a backdrop from the backdrop agent.
    pub fn open(&mut self, backdrop: Backdrop) {
        self.0.send(BackdropRequest::Open(backdrop))
    }

    /// Close the current backdrop.
    pub fn close(&mut self) {
        self.0.send(BackdropRequest::Close)
    }
}

impl Default for BackdropDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BackdropBridge(Box<dyn Bridge<Backdropper>>);

impl BackdropBridge {
    pub fn new(callback: Callback<<Backdropper as Agent>::Output>) -> Self {
        BackdropBridge(Backdropper::bridge(callback))
    }
}

// component

#[derive(Clone, PartialEq, Properties)]
pub struct Props {}

pub struct BackdropViewer {
    props: Props,
    _bridge: BackdropBridge,

    content: Html,
    open: bool,
}

pub enum Msg {
    Open(Backdrop),
    Close,
}

impl Component for BackdropViewer {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let bridge = BackdropBridge::new(link.callback(|action| match action {
            BackdropAction::Open(backdrop) => Msg::Open(backdrop),
            BackdropAction::Close => Msg::Close,
        }));
        Self {
            props,
            _bridge: bridge,
            content: Default::default(),
            open: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Open(content) => {
                self.content = content.content;
                self.open();
            }
            Msg::Close => {
                self.content = Default::default();
                self.close();
            }
        }
        true
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
        if self.open {
            return html! {
                <div class="pf-c-backdrop">
                    { self.content.clone() }
                </div>
            };
        } else {
            return html! {};
        }
    }
}

impl BackdropViewer {
    fn open(&mut self) {
        if let Some(body) = yew::utils::document().body() {
            let classes = js_sys::Array::of1(&JsValue::from_str("pf-c-backdrop__open"));
            body.class_list().add(&classes).ok();
        }
        self.open = true;
    }

    fn close(&mut self) {
        if let Some(body) = yew::utils::document().body() {
            let classes = js_sys::Array::of1(&JsValue::from_str("pf-c-backdrop__open"));
            body.class_list().remove(&classes).ok();
        }
        self.open = false;
    }
}
