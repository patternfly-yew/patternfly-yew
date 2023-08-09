use yew::prelude::*;

/// Create a new callback handling only the case when the user pressed the enter key.
#[hook]
pub fn use_on_enter<F, D>(f: F, d: D) -> Callback<KeyboardEvent>
where
    F: Fn(&D) + 'static,
    D: PartialEq + 'static,
{
    use_callback(
        move |evt: KeyboardEvent, d| {
            if evt.key_code() == 13 {
                f(d)
            }
        },
        d,
    )
}
