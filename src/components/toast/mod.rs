//! Toast notifications
use crate::prelude::{Action, Alert, AlertGroup, AlertType};
use chrono::{DateTime, Utc};
use core::cmp::Reverse;
use gloo_timers::callback::Timeout;
use std::{collections::BinaryHeap, time::Duration};
use yew::{prelude::*, virtual_dom::VChild};

/// Toasts are small alerts that get shown on the top right corner of the page.
///
/// A toast can be triggered by every component. The toast fill get sent to an agent, the Toaster.
/// The toaster will delegate displaying the toast to an instance of a ToastViewer component.
///
/// In order for Toasts to be displayed your application must have exactly one [ToastViewer](`ToastViewer`) **before**
/// creating the first Toast.
///
/// For example:
/// ```
/// # use yew::prelude::*;
/// # use patternfly_yew::prelude::*;
/// #[function_component(App)]
/// fn app() -> Html {
///   html! {
///     <>
///       <ToastViewer>
///         <View/>
///       </ToastViewer>
///     </>
///   }
/// }
/// #[function_component(View)]
/// fn view() -> Html {
///   let toaster = use_toaster().expect("Must be nested under a ToastViewer component");
///   html!{
///     <div>
///       <button onclick={move |_| toaster.toast("Toast Title".into())}>
///         { "Click me" }  
///       </button>
///     </div>
///   }
/// }
/// ```
#[derive(Clone, Debug, Default)]
pub struct Toast {
    pub title: String,
    pub r#type: AlertType,
    /// The timeout when the toast will be removed automatically.
    ///
    /// If no timeout is set, the toast will get a close button.
    pub timeout: Option<Duration>,
    pub body: Html,
    pub actions: Vec<Action>,
}

/// Allows to convert a string into a toast by using the string as title.
impl<S: ToString> From<S> for Toast {
    fn from(message: S) -> Self {
        Toast {
            title: message.to_string(),
            timeout: None,
            body: Default::default(),
            r#type: Default::default(),
            actions: Vec::new(),
        }
    }
}

#[doc(hidden)]
#[derive(Debug)]
pub enum ToasterRequest {
    Toast(Toast),
}

#[doc(hidden)]
pub enum ToastAction {
    ShowToast(Toast),
}

/// An agent for displaying toasts.
#[derive(Clone, PartialEq)]
pub struct Toaster {
    callback: Callback<ToastAction>,
}

impl Toaster {
    /// Request a toast from the toast viewer.
    pub fn toast(&self, toast: Toast) {
        self.callback.emit(ToastAction::ShowToast(toast))
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

pub struct ToastEntry {
    id: usize,
    alert: VChild<Alert>,
    timeout: Option<DateTime<Utc>>,
}

/// A component to view toast alerts.
///
/// Exactly one instance is required in your page in order to actually show the toasts. The instance
/// must be on the body level of the HTML document.
pub struct ToastViewer {
    context: Toaster,
    alerts: Vec<ToastEntry>,
    counter: usize,

    task: Option<Timeout>,
    timeouts: BinaryHeap<Reverse<DateTime<Utc>>>,
}

pub enum ToastViewerMsg {
    Perform(ToastAction),
    Cleanup,
    Close(usize),
}

impl Component for ToastViewer {
    type Message = ToastViewerMsg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let context = Toaster {
            callback: ctx.link().callback(ToastViewerMsg::Perform),
        };
        Self {
            context,
            alerts: Vec::new(),
            counter: 0,
            task: None,
            timeouts: BinaryHeap::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ToastViewerMsg::Perform(action) => self.perform(ctx, action),
            ToastViewerMsg::Cleanup => self.cleanup(ctx),
            ToastViewerMsg::Close(id) => self.remove_toast(id),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let context = self.context.clone();

        html! {
            <ContextProvider<Toaster> {context}>
                <AlertGroup toast=true>
                    { for self.alerts.iter().map(|entry|entry.alert.clone()) }
                </AlertGroup>
                { for ctx.props().children.iter() }
            </ContextProvider<Toaster>>
        }
    }
}

impl ToastViewer {
    fn now() -> DateTime<Utc> {
        Utc::now()
    }

    fn perform(&mut self, ctx: &Context<Self>, action: ToastAction) -> bool {
        match action {
            ToastAction::ShowToast(toast) => self.add_toast(ctx, toast),
        }
        true
    }

    fn add_toast(&mut self, ctx: &Context<Self>, toast: Toast) {
        let now = Self::now();
        let timeout = toast
            .timeout
            .and_then(|timeout| chrono::Duration::from_std(timeout).ok())
            .map(|timeout| now + timeout);

        let id = self.counter;
        self.counter += 1;

        let onclose = match toast.timeout {
            None => Some(ctx.link().callback(move |_| ToastViewerMsg::Close(id))),
            Some(_) => None,
        };

        self.alerts.push(ToastEntry {
            id,
            alert: html_nested! {
                <Alert r#type={toast.r#type} title={toast.title} onclose={onclose} actions={toast.actions}>
                    { toast.body }
                </Alert>
            },
            timeout,
        });

        if let Some(timeout) = timeout {
            self.schedule_cleanup(ctx, timeout);
        }
    }

    fn schedule_cleanup(&mut self, ctx: &Context<Self>, timeout: DateTime<Utc>) {
        log::debug!("Schedule cleanup: {:?}", timeout);

        self.timeouts.push(Reverse(timeout));
        self.trigger_next_cleanup(ctx);
    }

    fn trigger_next_cleanup(&mut self, ctx: &Context<Self>) {
        if self.task.is_some() {
            log::debug!("Already have a task");
            return;
        }

        // We poll timeouts from the heap until we find one that is in the future, or we run
        // out of candidates.
        while let Some(next) = self.timeouts.pop() {
            let timeout = next.0;
            log::debug!("Next timeout: {:?}", timeout);
            let duration = timeout - Self::now();
            let duration = duration.to_std();
            log::debug!("Duration: {:?}", duration);
            if let Ok(duration) = duration {
                let link = ctx.link().clone();
                self.task = Some(Timeout::new(duration.as_millis() as u32, move || {
                    link.send_message(ToastViewerMsg::Cleanup);
                }));
                log::debug!("Scheduled cleanup: {:?}", duration);
                break;
            }
        }
    }

    fn remove_toast(&mut self, id: usize) -> bool {
        self.retain_alert(|entry| entry.id != id)
    }

    fn cleanup(&mut self, ctx: &Context<Self>) -> bool {
        let now = Self::now();

        self.task = None;
        self.trigger_next_cleanup(ctx);

        self.retain_alert(|alert| {
            if let Some(timeout) = alert.timeout {
                timeout > now
            } else {
                true
            }
        })
    }

    fn retain_alert<F>(&mut self, f: F) -> bool
    where
        F: Fn(&ToastEntry) -> bool,
    {
        let before = self.alerts.len();
        self.alerts.retain(f);
        before != self.alerts.len()
    }
}

/// Get a [`Toaster`] context.
#[hook]
pub fn use_toaster() -> Option<Toaster> {
    use_context()
}
