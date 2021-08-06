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
    Default,
    // inline
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
    pub fn is_inline(&self) -> bool {
        matches!(self, Self::Inline)
    }
}

impl Default for ClipboardVariant {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Copy,
    Copied,
    Failed(&'static str),
    Reset,
    ToggleExpand,
}

const DEFAULT_MESSAGE: &'static str = "Copy to clipboard";
const FAILED_MESSAGE: &'static str = "Failed to copy";
const OK_MESSAGE: &'static str = "Copied!";

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
                self.trigger_message(OK_MESSAGE);
            }
            Msg::Failed(msg) => {
                self.trigger_message(msg);
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
        if self.props.variant.is_inline() {
            classes.push("pf-m-inline");
        }

        return html! {
            <div class=classes>
                { match self.props.variant {
                    ClipboardVariant::Inline => {
                        html!{
                            <>
                            {
                                if self.props.code {
                                    html!{<code class="pf-c-clipboard-copy__text pf-m-code">{&self.value}</code>}
                                } else {
                                    html!{<span class="pf-c-clipboard-copy__text">{&self.value}</span>}
                                }
                            }
                            <span class="pf-c-clipboard-copy__actions">
                                <span class="pf-c-clipboard-copy__actions-item">
                                    <Tooltip text=self.message>
                                        <Button aria_label="Copy to clipboard" variant=Variant::Plain icon=Icon::Copy onclick=self.link.callback(|_|Msg::Copy)/>
                                    </Tooltip>
                                </span>
                            </span>
                            </>
                        }
                    },
                    _ => {
                        html!{
                            <>
                            <div class="pf-c-clipboard-copy__group">
                                { self.expander() }
                                <TextInput ref=self.text_ref.clone() readonly={self.props.readonly | self.expanded} value=&self.value/>
                                <Tooltip text=self.message>
                                    <Button aria_label="Copy to clipboard" variant=Variant::Control icon=Icon::Copy onclick=self.link.callback(|_|Msg::Copy)/>
                                </Tooltip>
                            </div>
                            { self.expanded() }
                            </>
                        }
                    }
                }}
            </div>
        };
    }
}

impl Clipboard {
    fn trigger_message(&mut self, msg: &'static str) {
        self.message = msg;
        self.task.take();
        self.task = Some(Box::new(TimeoutService::spawn(
            Duration::from_secs(2),
            self.link.callback(|_| Msg::Reset),
        )));
    }

    fn do_copy(&self) {
        let s = self.value.clone();

        let ok: Callback<()> = self.link.callback(|_| Msg::Copied);
        let err: Callback<&'static str> = self.link.callback(|s| Msg::Failed(s));

        wasm_bindgen_futures::spawn_local(async move {
            match copy_to_clipboard(s).await {
                Ok(_) => ok.emit(()),
                Err(_) => err.emit(FAILED_MESSAGE),
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
        if self.props.readonly || self.props.variant.is_inline() {
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

#[wasm_bindgen(inline_js=r#"
export function copy_to_clipboard(value) {
    try {
        return window.navigator.clipboard.writeText(value);
    } catch(e) {
        console.log(e);
        return Promise.reject(e)
    }
}
"#)]
#[rustfmt::skip] // required to keep the "async" keyword
extern "C" { 
    #[wasm_bindgen(catch)]
    async fn copy_to_clipboard(value: String) -> Result<(), JsValue>;
}
