use crate::{Button, GlobalClose, Variant};
use yew::prelude::*;


#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub total_entries: Option<i32>,
    #[prop_or_default]
    pub offset: i32,
    #[prop_or(vec![10,20,30])]
    pub entries_per_page_choices: Vec<i32>,
    #[prop_or(20)]
    pub selected_choice: i32,


    // callback for the buttons
    #[prop_or_default]
    pub navigation_callback: Callback<Navigation>,

    #[prop_or_default]
    pub limit_callback: Callback<i32>,
}

pub enum Navigation {
    First,
    Previous,
    Next,
    Last
}

pub struct Pagination {
    expanded: bool,
    global_close: GlobalClose,
}

#[derive(Clone, Debug)]
pub enum Msg {
    ToggleMenu,
    CloseMenu,
    SetLimit(i32),

    First,
    Previous,
    Next,
    Last,
}

impl Component for Pagination {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            expanded: false,
            global_close: GlobalClose::new(NodeRef::default(), ctx.link().callback(|_| Msg::CloseMenu)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.expanded = !self.expanded;
            },
            Msg::SetLimit(limit) => ctx.props().limit_callback.emit(limit),
            Msg::CloseMenu => self.expanded = false,
            Msg::First => ctx.props().navigation_callback.emit(Navigation::First),
            Msg::Previous => ctx.props().navigation_callback.emit(Navigation::Previous),
            Msg::Next => ctx.props().navigation_callback.emit(Navigation::Next),
            Msg::Last => ctx.props().navigation_callback.emit(Navigation::Last),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        // The pagination menu : "1-20 of nnn"
        let mut menu_classes = Classes::from("pf-c-options-menu");
        if self.expanded {
            menu_classes.push("pf-m-expanded");
        }

        // The default rust div operator does floor(), we need ceil, so we cast to float before doing the operation
        let max_page = ctx.props().total_entries.map(|m| (m as f32 / ctx.props().selected_choice as f32).ceil() as i32);
        let current_page = (ctx.props().offset as f32 / ctx.props().selected_choice as f32).ceil() as i32;

        let is_last_page = if let Some(max) = ctx.props().total_entries {
            ctx.props().offset + ctx.props().selected_choice > max
        } else {
            // if max_page is not known, then displaying the "last page" button is pointless
            true
        };

        let total_entries = ctx.props().total_entries.map(|m| format!("{}", m)).unwrap_or(String::from("many"));
        // +1 because humans don't count from 0 :)
        let showing = format!("{} - {}", ctx.props().offset +1,  ctx.props().offset + ctx.props().selected_choice);

        let limit_choices = ctx.props().entries_per_page_choices.clone();
        let link = ctx.link().clone();

        return html! {

    <div class="pf-c-pagination" ref={self.global_close.clone()} >

        // the selector of how many entries per page to display
        <div class="pf-c-pagination__total-items">
            <b>{ showing.clone() }</b> {"\u{00a0}of\u{00a0}"}
            <b>{ total_entries.clone() }</b>
        </div>
        <div class={ menu_classes }>
            <div class="pf-c-options-menu__toggle pf-m-text pf-m-plain">
                <span class="pf-c-options-menu__toggle-text">
                     <b>{ showing }</b>{"\u{00a0}of\u{00a0}"}
                    <b>{ total_entries }</b>
                </span>
            <Button
                class="pf-c-options-menu__toggle-button"
                id="pagination-options-menu-top-toggle"
                //aria-haspopup="listbox"
                aria_label="Items per page"
                onclick={ctx.link().callback(|_|Msg::ToggleMenu)}
                >
                    <span class="pf-c-options-menu__toggle-button-icon">
                        <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
            </Button>
    </div>
    {{ if self.expanded {
        html! {
        <ul class="pf-c-options-menu__menu" >
            { for limit_choices.into_iter().map(|limit|  { html!{
                //todo add a check icon for the selected choice
                  <li>
                    <Button
                        class="pf-c-options-menu__menu-item"
                        onclick={link.callback(move |_|Msg::SetLimit(limit))}>
                            {limit} {" per page"}
                    </Button>
                  </li>
            }})}
        </ul>
        }
    } else { html! {} }
    }}
  </div>


        // the navigation buttons

        <nav class="pf-c-pagination__nav" aria-label="Pagination">
            <div class="pf-c-pagination__nav-control pf-m-first">
              <Button
                variant={Variant::InlineLink}
                onclick={ctx.link().callback(|_|Msg::First)}
                disabled={ ctx.props().offset == 0 }
                aria_label="Go to first page"
              >
                <i class="fas fa-angle-double-left" aria-hidden="true"></i>
              </Button>
            </div>
            <div class="pf-c-pagination__nav-control pf-m-prev">
                <Button
                    aria_label="Go to previous page"
                    variant={Variant::Plain}
                    onclick={ctx.link().callback(|_|Msg::Previous)}
                    disabled= { ctx.props().offset == 0 }
                >
                        <i class="fas fa-angle-left" aria-hidden="true"></i>
                </Button>
            </div>
            <div class="pf-c-pagination__nav-page-select">
              <input
                class="pf-c-form-control"
                aria-label="Current page"
                type="number"
                min="1"
                // todo conditionnally add max field if known
                //max = max_page
                value = {current_page.to_string()}
              />
            {{
                if let Some(max_page) = max_page {
                    html!{<span aria-hidden="true">{ "of "} { max_page }</span>}
                } else {
                    html!{}
                }
            }}
            </div>
            <div class="pf-c-pagination__nav-control pf-m-next">
              <Button
                aria_label="Go to next page"
                variant={Variant::InlineLink}
                onclick={ctx.link().callback(|_|Msg::Next)}
                disabled={max_page.map_or_else(|| false, |m| current_page >= m)}
              >
                <i class="fas fa-angle-right" aria-hidden="true"></i>
              </Button>
            </div>
            <div class="pf-c-pagination__nav-control pf-m-last">
              <Button
                aria_label="Go to last page"
                variant={Variant::InlineLink}
                onclick={ctx.link().callback(|_|Msg::Last)}
                disabled={is_last_page}
              >
                <i class="fas fa-angle-double-right" aria-hidden="true"></i>
              </Button>
            </div>
        </nav>
    </div>
        };
    }
}

