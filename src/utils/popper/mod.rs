pub(crate) use crate::integration::popperjs::Options as PopperOptions;
use crate::integration::popperjs::*;
pub use crate::integration::popperjs::{
    Modifier, Offset, Options, Placement as PopperPlacement, PreventOverflow, State as PopperState,
    Strategy as PopperStrategy,
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PopperProperties {
    pub visible: bool,

    /// The children to which the content should be attached.
    #[prop_or_default]
    pub children: Children,

    /// The content show then the popper is visible.
    pub content: Html,

    pub mode: PopperMode,

    #[prop_or_default]
    pub onstatechange: Callback<PopperState>,

    #[prop_or_default]
    pub options: PopperOptions,

    #[prop_or_default]
    pub target_ref: Option<NodeRef>,

    #[prop_or_default]
    pub content_ref: Option<NodeRef>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PopperMode {
    Inline,
    Portal,
}

struct PopperInstance {
    id: Uuid,
    instance: Instance,
    /// keep the callback so to that it doesn't get dropped
    _callback: Closure<dyn Fn(&Instance)>,
}

impl PartialEq for PopperInstance {
    fn eq(&self, other: &Self) -> bool {
        // we define equality by the id, considering this as an instance id
        self.id == other.id
    }
}

#[function_component(Popper)]
pub fn popper(props: &PopperProperties) -> Html {
    let ready = use_state_eq(|| false);

    let target_ref = use_node_ref();
    let content_ref = use_node_ref();

    let (wrapping_target, target_ref) = match &props.target_ref {
        Some(target_ref) => (false, target_ref.clone()),
        None => (true, target_ref),
    };

    let (wrapping_content, content_ref) = match &props.content_ref {
        Some(content_ref) => (false, content_ref.clone()),
        None => (true, content_ref),
    };

    let content = use_memo(
        |(content, mode, content_ref, wrapping)| {
            let content = match wrapping {
                true => {
                    html!(
                        <div ref={content_ref}>
                            { (*content).clone() }
                        </div>
                    )
                }
                false => (*content).clone(),
            };

            match mode {
                PopperMode::Inline => content,
                PopperMode::Portal => create_portal(content, gloo_utils::body().into()),
            }
        },
        (
            props.content.clone(),
            props.mode,
            content_ref.clone(),
            wrapping_content,
        ),
    );

    let popper = use_memo(
        |(options, target, onstatechange, content, ready)| {
            log::debug!("Ready: {}", *ready);
            //log::debug!("Target: {target:?}");
            //log::debug!("Content: {content:?}");

            match (*ready, target.get(), content.get()) {
                (true, Some(target), Some(content)) => {
                    log::debug!("Creating popper instance");
                    let callback = {
                        let onstatechange = onstatechange.clone();
                        Closure::wrap(Box::new(move |this: &Instance| {
                            web_sys::console::debug_2(&JsValue::from("apply: "), this);
                            if let Ok(state) = from_popper(this) {
                                onstatechange.emit(state);
                            }
                        }) as Box<dyn Fn(&Instance)>)
                    };

                    let opts = create_opts(&callback, options.clone()).unwrap();

                    Some(PopperInstance {
                        id: Uuid::new_v4(),
                        instance: create_popper(target, content, &opts),
                        _callback: callback,
                    })
                }
                _ => None,
            }
        },
        (
            props.options.clone(),
            target_ref.clone(),
            props.onstatechange.clone(),
            content_ref.clone(),
            *ready,
        ),
    );

    // Require a re-render to get node ref filled
    ready.set(props.visible);

    if let Some(popper) = &*popper {
        popper.instance.force_update();
    }

    use_effect_with_deps(
        |popper| {
            let popper = popper.clone();
            move || {
                if let Some(popper) = &*popper {
                    log::debug!("Destroying popper instance");
                    popper.instance.destroy();
                }
            }
        },
        popper.clone(),
    );

    html! (
        <>
            if wrapping_target {
                <div ref={target_ref}>
                    { for props.children.iter() }
                </div>
            } else {
                { for props.children.iter() }
            }

            if props.visible {
                { (*content).clone() }
            }
        </>
    )
}
