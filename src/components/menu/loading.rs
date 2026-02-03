use crate::components::spinner::Spinner;
use crate::prelude::SpinnerSize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MenuLoadingProperties {}

#[function_component(MenuLoading)]
pub fn menu_loading(_props: &MenuLoadingProperties) -> Html {
    let class = classes!("pf-v6-c-menu__list-item", "pf-m-loading");

    html!(
        <li {class} role="none">
            <button class="pf-v6-c-menu__item" type="button" role="menuitem">
                <span class="pf-v6-c-menu__item-main">
                    <span class="pf-v6-c-menu__item-text">
                        <Spinner size={SpinnerSize::Lg} />
                    </span>
                </span>
            </button>
        </li>
    )
}
