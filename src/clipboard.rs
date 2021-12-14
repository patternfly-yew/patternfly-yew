use crate::button::*;
use crate::form::*;
use crate::icon::*;
use crate::*;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

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
    #[prop_or_default]
    pub name: String,
    #[prop_or_default]
    pub id: String,
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
    message: &'static str,
    task: Option<Timeout>,
    expanded: bool,
    value: String,
    text_ref: NodeRef,
    details_ref: NodeRef,
}

impl Component for Clipboard {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let expanded = match ctx.props().variant {
            ClipboardVariant::Expanded => true,
            _ => false,
        };

        let value = ctx.props().value.clone();

        Self {
            message: DEFAULT_MESSAGE,
            task: None,
            expanded,
            value,
            text_ref: NodeRef::default(),
            details_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Copy => {
                self.sync_value(ctx);
                self.do_copy(ctx);
            }
            Msg::Copied => {
                self.trigger_message(ctx, OK_MESSAGE);
            }
            Msg::Failed(msg) => {
                self.trigger_message(ctx, msg);
            }
            Msg::Reset => {
                self.message = DEFAULT_MESSAGE;
                self.task.take();
            }
            Msg::ToggleExpand => {
                self.sync_value(ctx);
                self.expanded = !self.expanded;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-clipboard-copy");

        if self.expanded {
            classes.push("pf-m-expanded");
        }
        if ctx.props().variant.is_inline() {
            classes.push("pf-m-inline");
        }

        return html! {
            <div class={classes}>
                { match ctx.props().variant {
                    ClipboardVariant::Inline => {
                        html!{
                            <>
                            if ctx.props().code {
                                <code name={ctx.props().name.clone()} id={ctx.props().id.clone()} class="pf-c-clipboard-copy__text pf-m-code">{&self.value}</code>
                            } else {
                                <span name={ctx.props().name.clone()} id={ctx.props().id.clone()} class="pf-c-clipboard-copy__text">{&self.value}</span>
                            }
                            <span class="pf-c-clipboard-copy__actions">
                                <span class="pf-c-clipboard-copy__actions-item">
                                    <Tooltip text={self.message}>
                                        <Button aria_label="Copy to clipboard" variant={Variant::Plain} icon={Icon::Copy} onclick={ctx.link().callback(|_|Msg::Copy)}/>
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
                                { self.expander(ctx) }
                                <TextInput
                                    ref={self.text_ref.clone()}
                                    readonly={ctx.props().readonly | self.expanded}
                                    value={self.value.clone()}
                                    name={ctx.props().name.clone()}
                                    id={ctx.props().id.clone()}
                                />
                                <Tooltip text={self.message}>
                                    <Button aria_label="Copy to clipboard" variant={Variant::Control} icon={Icon::Copy} onclick={ctx.link().callback(|_|Msg::Copy)}/>
                                </Tooltip>
                            </div>
                            { self.expanded(ctx) }
                            </>
                        }
                    }
                }}
            </div>
        };
    }
}

impl Clipboard {
    fn trigger_message(&mut self, ctx: &Context<Self>, msg: &'static str) {
        self.message = msg;
        self.task = Some({
            let link = ctx.link().clone();
            Timeout::new(2_000, move || {
                link.send_message(Msg::Reset);
            })
        });
    }

    fn do_copy(&self, ctx: &Context<Self>) {
        let s = self.value.clone();

        let ok: Callback<()> = ctx.link().callback(|_| Msg::Copied);
        let err: Callback<&'static str> = ctx.link().callback(|s| Msg::Failed(s));

        wasm_bindgen_futures::spawn_local(async move {
            match copy_to_clipboard(s).await {
                Ok(_) => ok.emit(()),
                Err(_) => err.emit(FAILED_MESSAGE),
            };
        });
    }

    fn expander(&self, ctx: &Context<Self>) -> Html {
        if !ctx.props().variant.is_expandable() {
            return Default::default();
        }

        let onclick = ctx.link().callback(|_| Msg::ToggleExpand);

        return html! {
            <Button
                expanded={self.expanded}
                variant={Variant::Control}
                onclick={onclick}>
                <div class="pf-c-clipboard-copy__toggle-icon">
                    { Icon::AngleRight }
                </div>
            </Button>
        };
    }

    fn expanded(&self, ctx: &Context<Self>) -> Html {
        if !self.expanded {
            return Default::default();
        }

        return html! {
            <div
                ref={self.details_ref.clone()}
                class="pf-c-clipboard-copy__expandable-content"
                contenteditable={(!ctx.props().readonly).to_string()}>

                { if ctx.props().code {
                    html!{ <pre>{&self.value}</pre> }
                } else {
                    html!{ &self.value}
                } }

            </div>
        };
    }

    /// Sync the value between internal, text field or details.
    fn sync_value(&mut self, ctx: &Context<Self>) {
        if ctx.props().readonly || ctx.props().variant.is_inline() {
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
