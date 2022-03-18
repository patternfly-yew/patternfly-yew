use crate::{Avatar, Button, Divider, GlobalClose, Icon, Nav, Position, Variant};
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};


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


    // callbacks for the buttons
    #[prop_or_default]
    pub on_first: Callback<()>,
    #[prop_or_default]
    pub on_previous: Callback<()>,
    #[prop_or_default]
    pub on_next: Callback<()>,
    #[prop_or_default]
    pub on_last: Callback<()>,
}

pub struct Pagination {
    expanded: bool,
}

#[derive(Clone, Debug)]
pub enum Msg {
    ToggleMenu,
    CloseMenu,

    First,
    Previous,
    Next,
    Last,
}

impl Component for Pagination {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {
            expanded: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.expanded = !self.expanded;
            }
            Msg::CloseMenu => self.expanded = false,
            Msg::First => ctx.props().on_first.emit(()),
            Msg::Previous => ctx.props().on_previous.emit(()),
            Msg::Next => ctx.props().on_next.emit(()),
            Msg::Last => ctx.props().on_last.emit(()),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-pagination");

        // The pagination menu : "1-20 of nnn"
        let mut menu_classes = Classes::from("pf-c-options-menu");
        if self.expanded {
            menu_classes.push("pf-c-options-menu pf-m-expanded");
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
        let showing = format!("{} - {}", ctx.props().offset,  ctx.props().offset + ctx.props().selected_choice);

        return html! {

    <div class="pf-c-pagination">

        // the selector of how many entries per page to display
        <div class="pf-c-pagination__total-items">
            <b>{ showing.clone() }</b> {"\u{00a0}of\u{00a0}"}
            <b>{ total_entries.clone() }</b>
        </div>
        <div class="pf-c-options-menu">
            <div class="pf-c-options-menu__toggle pf-m-text pf-m-plain">
                <span class="pf-c-options-menu__toggle-text">
                     <b>{ showing }</b>{"\u{00a0}of\u{00a0}"}
                    <b>{ total_entries }</b>
                </span>
            <Button
                class="pf-c-options-menu__toggle-button"
                id="pagination-options-menu-top-example-toggle"
                //aria-haspopup="listbox"
                aria_label="Items per page"
                onclick={ctx.link().callback(|_|Msg::ToggleMenu)}
                >
                    <span class="pf-c-options-menu__toggle-button-icon">
                        <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
            </Button>
    </div>
    <ul
      class="pf-c-options-menu__menu"
      aria-labelledby= { if self.expanded {
                "pagination-options-menu-top-example-toggle"
                    } else {
                        "pagination-options-menu-top-expanded-example-toggle"
                    }
            }
      //hidden
    >
            //TODO : bubble up the choice to the parent component when clicked
    { for ctx.props().entries_per_page_choices.iter().map(|i| html!{
              <li>
                <Button class="pf-c-options-menu__menu-item">
                    {i} {" per page"}
                </Button>
              </li>
    })}
    </ul>
  </div>


        // the navigation buttons

        <Nav>
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
        </Nav>
    </div>
        };
    }
}

