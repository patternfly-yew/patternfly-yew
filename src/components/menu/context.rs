use yew::prelude::*;

/// A context to request a menu to close.
///
/// This is intended to be used by components implementing a drop-down style menu, where clicking
/// on a menu entry is intended to close the menu.
#[derive(Clone, PartialEq)]
pub struct CloseMenuContext {
    onclose: Callback<()>,
}

impl CloseMenuContext {
    pub fn new(onclose: Callback<()>) -> Self {
        Self { onclose }
    }

    /// Close the expanded menu
    pub fn close(&self) {
        self.onclose.emit(());
    }
}

/// Access the menu context.
///
/// This will only return a non-none value when called from a component nested in a component
/// supporting this context.
#[hook]
pub fn use_close_menu_context() -> Option<CloseMenuContext> {
    use_context()
}

#[derive(Clone, PartialEq)]
pub struct UseCloseMenu {
    pub context: Option<CloseMenuContext>,
}

impl UseCloseMenu {
    pub fn close(&self) {
        if let Some(context) = &self.context {
            context.close();
        } else {
            log::warn!("Ignored request to close menu: no context was found")
        }
    }
}

/// Allow closing the menu.
///
/// **NOTE**: If the hook is used inside a component which is not wrapped with a component
/// providing a [`CloseMenuContext`], then all operations become a no-op.
#[hook]
pub fn use_close_menu() -> UseCloseMenu {
    UseCloseMenu {
        context: use_context(),
    }
}

/// Provide a stable callback for closing the menu.
///
/// **NOTE**: If the hook is used inside a component which is not wrapped with a component
/// providing a [`CloseMenuContext`], then all operations become a no-op.
#[hook]
pub fn use_close_menu_callback() -> Callback<()> {
    let context = use_close_menu();
    use_callback(context, |(), context| context.close())
}
