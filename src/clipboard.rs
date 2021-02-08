use std::time::Duration;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{Task, TimeoutService};

use crate::button::*;
use crate::form::*;
use crate::icon::*;
use crate::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub readonly: bool,
}

#[derive(Clone, Debug)]
pub enum Msg {
    Copy,
    Copied,
    Reset,
}

const DEFAULT_MESSAGE: &'static str = "Copy to clipboard";

pub struct Clipboard {
    props: Props,
    link: ComponentLink<Self>,
    message: &'static str,
    task: Option<Box<dyn Task>>,
}

impl Component for Clipboard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            message: DEFAULT_MESSAGE,
            task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Copy => self.do_copy(),
            Msg::Copied => {
                log::info!("Copied");
                self.message = "Copied!";
                self.task = Some(Box::new(TimeoutService::spawn(
                    Duration::from_secs(2),
                    self.link.callback(|_| Msg::Reset),
                )));
            }
            Msg::Reset => {
                self.message = DEFAULT_MESSAGE;
                self.task.take();
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
        html! {
            <div class="pf-c-clipboard-copy">
                <div class="pf-c-clipboard-copy__group">
                    <TextInput readonly=self.props.readonly value=&self.props.value/>
                    <Tooltip text=self.message>
                        <Button variant=Variant::Control icon=Icon::Copy onclick=self.link.callback(|_|Msg::Copy)/>
                    </Tooltip>
                </div>
            </div>
        }
    }
}

impl Clipboard {
    fn do_copy(&self) {
        let s = self.props.value.clone();

        let cb: Callback<()> = self.link.callback(|_| Msg::Copied);

        wasm_bindgen_futures::spawn_local(async move {
            match copy_to_clipboard(s).await {
                Ok(_) => cb.emit(()),
                Err(_) => {}
            };
        });
    }
}

#[wasm_bindgen(inline_js="export function copy_to_clipboard(value) {return window.navigator.clipboard.writeText(value);}")]
#[rustfmt::skip] // required to keep the "async" keyword
extern "C" { 
    #[wasm_bindgen(catch)]
    async fn copy_to_clipboard(value: String) -> Result<(), JsValue>;
}
