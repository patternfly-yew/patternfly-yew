use std::time::Duration;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::services::{Task, TimeoutService};

use crate::button::*;
use crate::form::*;
use crate::icon::*;
use crate::*;
use yew::web_sys::{Element, HtmlInputElement};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub value: String,
    #[prop_or_default]
    pub readonly: bool,
    #[prop_or_default]
    pub code: bool,
    #[prop_or_default]
    pub variant: ClipboardVariant,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ClipboardVariant {
    // default
    Inline,
    // expandable
    Expandable,
    // expandable and initially expanded
    Expanded,
}

impl ClipboardVariant {
    pub fn is_expandable(&self) -> bool {
        match self {
            Self::Expandable | Self::Expanded => true,
            _ => false,
        }
    }
}

impl Default for ClipboardVariant {
    fn default() -> Self {
        Self::Inline
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Copy,
    Copied,
    Reset,
    ToggleExpand,
}

const DEFAULT_MESSAGE: &'static str = "Copy to clipboard";

pub struct Clipboard {
    props: Props,
    link: ComponentLink<Self>,
    message: &'static str,
    task: Option<Box<dyn Task>>,
    expanded: bool,
    value: String,
    text_ref: NodeRef,
    details_ref: NodeRef,
}

impl Component for Clipboard {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let expanded = match props.variant {
            ClipboardVariant::Expanded => true,
            _ => false,
        };

        let value = props.value.clone();

        Self {
            props,
            link,
            message: DEFAULT_MESSAGE,
            task: None,
            expanded,
            value,
            text_ref: NodeRef::default(),
            details_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Copy => {
                self.sync_value();
                self.do_copy();
            }
            Msg::Copied => {
                // log::info!("Copied");
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
            Msg::ToggleExpand => {
                self.sync_value();
                self.expanded = !self.expanded;
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.value = props.value.clone();
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-clipboard-copy");

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        return html! {
            <div class=classes>
                <div class="pf-c-clipboard-copy__group">
                    { self.expander() }
                    <TextInput ref=self.text_ref.clone() readonly={self.props.readonly | self.expanded} value=&self.value/>
                    <Tooltip text=self.message>
                        <Button variant=Variant::Control icon=Icon::Copy onclick=self.link.callback(|_|Msg::Copy)/>
                    </Tooltip>
                </div>
                { self.expanded() }
            </div>
        };
    }
}

impl Clipboard {
    fn do_copy(&self) {
        let s = self.value.clone();

        let cb: Callback<()> = self.link.callback(|_| Msg::Copied);

        wasm_bindgen_futures::spawn_local(async move {
            match copy_to_clipboard(s).await {
                Ok(_) => cb.emit(()),
                Err(_) => {}
            };
        });
    }

    fn expander(&self) -> Html {
        if !self.props.variant.is_expandable() {
            return Default::default();
        }

        let onclick = self.link.callback(|_| Msg::ToggleExpand);

        return html! {
            <Button
                expanded=self.expanded
                variant=Variant::Control
                onclick=onclick>
                <div class="pf-c-clipboard-copy__toggle-icon">
                    { Icon::AngleRight }
                </div>
            </Button>
        };
    }

    fn expanded(&self) -> Html {
        if !self.expanded {
            return Default::default();
        }

        return html! {
            <div
                ref=self.details_ref.clone()
                class="pf-c-clipboard-copy__expandable-content"
                contenteditable=!self.props.readonly>

                { if self.props.code {
                    html!{ <pre>{&self.value}</pre> }
                } else {
                    html!{ &self.value}
                } }

            </div>
        };
    }

    /// Sync the value between internal, text field or details.
    fn sync_value(&mut self) {
        if self.props.readonly {
            return;
        }

        let value = if !self.expanded {
            let ele: Option<HtmlInputElement> = self.text_ref.cast::<HtmlInputElement>();
            ele.map(|ele| ele.value()).unwrap_or_else(|| "".into())
        } else {
            let ele: Option<Element> = self.details_ref.cast::<Element>();
            ele.and_then(|ele| ele.text_content())
                .unwrap_or_else(|| "".into())
        };

        // log::info!("New value: {}", value);

        self.value = value;
    }
}

#[wasm_bindgen(inline_js="export function copy_to_clipboard(value) {return window.navigator.clipboard.writeText(value);}")]
#[rustfmt::skip] // required to keep the "async" keyword
extern "C" { 
    #[wasm_bindgen(catch)]
    async fn copy_to_clipboard(value: String) -> Result<(), JsValue>;
}
