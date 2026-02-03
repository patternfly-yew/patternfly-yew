//! Copy clipboard
use crate::icon::*;
use crate::prelude::TextInput;
use crate::prelude::*;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::{Element, HtmlInputElement};
use yew::prelude::*;

/// Properties for [``Clipboard]
#[derive(Clone, PartialEq, Properties)]
pub struct ClipboardProperties {
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

#[derive(Clone, Default, PartialEq, Eq, Debug)]
pub enum ClipboardVariant {
    // default
    #[default]
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
        matches!(self, Self::Expandable | Self::Expanded)
    }

    pub fn is_inline(&self) -> bool {
        matches!(self, Self::Inline)
    }
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum Msg {
    Copy,
    Copied,
    Failed(&'static str),
    Reset,
    ToggleExpand,
    /// Sync the content from the
    Sync,
}

const DEFAULT_MESSAGE: &str = "Copy to clipboard";
const FAILED_MESSAGE: &str = "Failed to copy";
const OK_MESSAGE: &str = "Copied!";

/// Clipboard copy component
///
/// > The **clipboard copy** component allows users to quickly and easily copy content to their clipboard.
///
/// See: <https://www.patternfly.org/components/clipboard-copy>
///
/// ## Properties
///
/// Defined by [`ClipboardProperties`].
pub struct Clipboard {
    message: &'static str,
    task: Option<Timeout>,
    expanded: bool,
    // the value, when overridden by the user
    value: Option<String>,
    text_ref: NodeRef,
    details_ref: NodeRef,
}

impl Component for Clipboard {
    type Message = Msg;
    type Properties = ClipboardProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let expanded = matches!(ctx.props().variant, ClipboardVariant::Expanded);

        Self {
            message: DEFAULT_MESSAGE,
            task: None,
            expanded,
            value: None,
            text_ref: NodeRef::default(),
            details_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Copy => {
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
                self.expanded = !self.expanded;
            }
            Msg::Sync => {
                self.sync_from_edit(ctx);
                return false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-v6-c-clipboard-copy");

        if self.expanded {
            classes.push("pf-m-expanded");
        }
        if ctx.props().variant.is_inline() {
            classes.push("pf-m-inline");
        }

        let value = self.value(ctx);

        html! {
            <div class={classes}>
                { match ctx.props().variant {
                    ClipboardVariant::Inline => {
                        html!{
                            <>
                            if ctx.props().code {
                                <code name={ctx.props().name.clone()} id={ctx.props().id.clone()} class="pf-v6-c-clipboard-copy__text pf-m-code">{value}</code>
                            } else {
                                <span name={ctx.props().name.clone()} id={ctx.props().id.clone()} class="pf-v6-c-clipboard-copy__text">{value}</span>
                            }
                            <span class="pf-v6-c-clipboard-copy__actions">
                                <span class="pf-v6-c-clipboard-copy__actions-item">
                                    <Tooltip text={self.message}>
                                        <Button aria_label="Copy to clipboard" variant={ButtonVariant::Plain} icon={Icon::Copy} onclick={ctx.link().callback(|_|Msg::Copy)}/>
                                    </Tooltip>
                                </span>
                            </span>
                            </>
                        }
                    },
                    _ => {
                        html!{
                            <>
                            <div class="pf-v6-c-clipboard-copy__group">
                                { self.expander(ctx) }
                                <TextInput
                                    r#ref={self.text_ref.clone()}
                                    readonly={ctx.props().readonly | self.expanded}
                                    value={value}
                                    name={ctx.props().name.clone()}
                                    id={ctx.props().id.clone()}
                                    oninput={ctx.link().callback(|_|Msg::Sync)}
                                />
                                <Tooltip text={self.message}>
                                    <Button aria_label="Copy to clipboard" variant={ButtonVariant::Control} icon={Icon::Copy} onclick={ctx.link().callback(|_|Msg::Copy)}/>
                                </Tooltip>
                            </div>
                            { self.expanded(ctx) }
                            </>
                        }
                    }
                }}
            </div>
        }
    }
}

impl Clipboard {
    fn value(&self, ctx: &Context<Self>) -> String {
        self.value
            .clone()
            .unwrap_or_else(|| ctx.props().value.clone())
    }

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
        let s = self.value(ctx);

        let ok: Callback<()> = ctx.link().callback(|_| Msg::Copied);
        let err: Callback<&'static str> = ctx.link().callback(Msg::Failed);

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

        html! (
            <Button
                expanded={self.expanded}
                variant={ButtonVariant::Control}
                onclick={onclick}>
                <div class="pf-v6-c-clipboard-copy__toggle-icon">
                    { Icon::AngleRight }
                </div>
            </Button>
        )
    }

    fn expanded(&self, ctx: &Context<Self>) -> Html {
        if !self.expanded {
            return Default::default();
        }

        let value = self.value(ctx);

        html! {
            <div
                ref={self.details_ref.clone()}
                class="pf-v6-c-clipboard-copy__expandable-content"
                contenteditable={(!ctx.props().readonly).to_string()}
                oninput={ctx.link().callback(|_|Msg::Sync)}
            >

                if ctx.props().code {
                    <pre>{ value }</pre>
                } else {
                    { value }
                }

            </div>
        }
    }

    /// Sync the value between internal, text field or details.
    fn sync_from_edit(&mut self, ctx: &Context<Self>) {
        if ctx.props().readonly || ctx.props().variant.is_inline() {
            return;
        }

        let value = if self.expanded {
            // from div to input
            let ele: Option<Element> = self.details_ref.cast::<Element>();
            ele.and_then(|ele| ele.text_content())
                .unwrap_or_else(|| "".into())
        } else {
            // from input to div
            let ele: Option<HtmlInputElement> = self.text_ref.cast::<HtmlInputElement>();
            ele.map(|ele| ele.value()).unwrap_or_else(|| "".into())
        };

        log::debug!("New value: {}", value);

        // sync back
        match self.expanded {
            true => {
                if let Some(ele) = self.text_ref.cast::<HtmlInputElement>() {
                    ele.set_value(&value);
                }
            }
            false => {
                if let Some(ele) = self.details_ref.cast::<Element>() {
                    ele.set_text_content(Some(&value));
                }
            }
        }

        // sync to internal state

        self.value = Some(value);
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
