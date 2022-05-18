use crate::{
    Button, GlobalClose, Icon, InputState, TextInput, ValidationContext, Validator, Variant,
};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub total_entries: Option<u32>,
    #[prop_or_default]
    pub offset: u32,
    #[prop_or(vec![10,20,30])]
    pub entries_per_page_choices: Vec<u32>,
    #[prop_or(20)]
    pub selected_choice: u32,

    // callback for the buttons
    #[prop_or_default]
    pub navigation_callback: Callback<Navigation>,

    #[prop_or_default]
    pub limit_callback: Callback<u32>,
}

pub enum Navigation {
    First,
    Previous,
    Next,
    Last,
    Page(u32),
}

pub struct Pagination {
    expanded: bool,
    global_close: GlobalClose,
}

#[derive(Clone, Debug)]
pub enum Msg {
    ToggleMenu,
    CloseMenu,
    SetLimit(u32),

    First,
    Previous,
    Next,
    Last,
    Page(u32),
}

impl Component for Pagination {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            expanded: false,
            global_close: GlobalClose::new(
                NodeRef::default(),
                ctx.link().callback(|_| Msg::CloseMenu),
            ),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ToggleMenu => {
                self.expanded = !self.expanded;
            }
            Msg::SetLimit(limit) => {
                ctx.props().limit_callback.emit(limit);
                ctx.link().send_message(Msg::CloseMenu)
            }
            Msg::CloseMenu => self.expanded = false,
            Msg::First => ctx.props().navigation_callback.emit(Navigation::First),
            Msg::Previous => ctx.props().navigation_callback.emit(Navigation::Previous),
            Msg::Next => ctx.props().navigation_callback.emit(Navigation::Next),
            Msg::Last => ctx.props().navigation_callback.emit(Navigation::Last),
            Msg::Page(num) => ctx.props().navigation_callback.emit(Navigation::Page(num)),
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
        let max_page = ctx
            .props()
            .total_entries
            .map(|m| (m as f64 / ctx.props().selected_choice as f64).ceil() as u32);
        let current_page =
            (ctx.props().offset as f64 / ctx.props().selected_choice as f64).ceil() as u32;

        let is_last_page = if let Some(max) = ctx.props().total_entries {
            ctx.props().offset + ctx.props().selected_choice > max
        } else {
            false
        };

        let total_entries = ctx
            .props()
            .total_entries
            .map(|m| format!("{}", m))
            .unwrap_or_else(|| String::from("unknown"));
        // +1 because humans don't count from 0 :)
        let showing = format!(
            "{} - {}",
            ctx.props().offset + 1,
            ctx.props().offset + ctx.props().selected_choice
        );

        let limit_choices = ctx.props().entries_per_page_choices.clone();
        let link = ctx.link().clone();

        // todo also add max page
        let page_number_field_validator = Validator::from(
            |ctx: ValidationContext<String>| match ctx.value.parse::<u32>() {
                Ok(value) => {
                    if value > 0 {
                        InputState::Default
                    } else {
                        InputState::Error
                    }
                }
                Err(_) => InputState::Error,
            },
        );

        let validator_clone = page_number_field_validator.clone();
        let onchange_callback = {
            ctx.link().callback(move |input: String| {
                match validator_clone.run(ValidationContext::from(input.clone())) {
                    Some(InputState::Default) => Msg::Page(input.parse::<u32>().unwrap()),
                    _ => Msg::Page(current_page + 1),
                }
            })
        };

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
                              { Icon::CaretDown }
                          </span>
                  </Button>
          </div>
          {{ if self.expanded {
              html! {
              <ul class="pf-c-options-menu__menu" >
                  { for limit_choices.into_iter().map(|limit|  { html!{
                        <li>
                          <Button
                              class="pf-c-options-menu__menu-item"
                                      onclick={link.callback(move |_|Msg::SetLimit(limit))}>
                                  {limit} {" per page"}
                                  {{ if ctx.props().selected_choice == limit {
                                       html!{
                                      <div class="pf-c-options-menu__menu-item-icon">
                                         { Icon::Check }
                                      </div>
                                          }
                                  } else {
                                      html!{}
                                  } }}
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
                      variant={Variant::Plain}
                      onclick={ctx.link().callback(|_|Msg::First)}
                      disabled={ ctx.props().offset == 0 }
                      aria_label="Go to first page"
                    >
                      { Icon::AngleDoubleLeft }
                    </Button>
                  </div>
                  <div class="pf-c-pagination__nav-control pf-m-prev">
                      <Button
                          aria_label="Go to previous page"
                          variant={Variant::Plain}
                          onclick={ctx.link().callback(|_|Msg::Previous)}
                          disabled= { ctx.props().offset == 0 }
                      >
                             { Icon::AngleLeft }
                      </Button>
                  </div>
                  <div class="pf-c-pagination__nav-page-select">
                    <TextInput
                      r#type="number"
                      validator={page_number_field_validator}
                      onchange={onchange_callback}
                      value = {(current_page+1).to_string()}
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
                      variant={Variant::Plain}
                      onclick={ctx.link().callback(|_|Msg::Next)}
                      disabled={max_page.map_or_else(|| false, |m| current_page >= m)}
                    >
                      { Icon::AngleRight }
                    </Button>
                  </div>
                  <div class="pf-c-pagination__nav-control pf-m-last">
                    <Button
                      aria_label="Go to last page"
                      variant={Variant::Plain}
                      onclick={ctx.link().callback(|_|Msg::Last)}
                      disabled={is_last_page}
                    >
                      { Icon::AngleDoubleRight }
                    </Button>
                  </div>
              </nav>
          </div>
              };
    }
}
