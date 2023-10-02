use crate::utils::value;
use yew::prelude::*;

/// A hook to handle the discrepancy between `oninput`/`onchange` for text input fields.
///
/// The hook will return a callback which is suitable for the `oninput` event of a text input. It
/// will emit the `oninput` event unchanged, and emit an `onchange` event with the full value
/// of the input control.
///
/// The `node` parameter must point to an HTML input element, suitable for the [`value`] function.
/// Otherwise no `onchange` event fill be fired.
#[hook]
pub fn use_on_text_change(
    node: NodeRef,
    oninput: Callback<InputEvent>,
    onchange: Callback<String>,
) -> Callback<InputEvent> {
    use_callback(
        (node, oninput, onchange),
        |evt, (node, oninput, onchange)| {
            oninput.emit(evt);
            if let Some(value) = value(node) {
                onchange.emit(value);
            }
        },
    )
}
