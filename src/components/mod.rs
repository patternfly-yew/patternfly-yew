//! Components

pub mod about;
pub mod alert;
pub mod app_launcher;
pub mod avatar;
pub mod backdrop;
pub mod background;
pub mod badge;
pub mod brand;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod chip;
pub mod chip_group;
pub mod clipboard;
pub mod code_block;
pub mod content;
pub mod context_selector;
pub mod divider;
pub mod dl;
pub mod dropdown;
pub mod empty;
pub mod expandable_section;
pub mod file_upload;
pub mod form;
pub mod helper_text;
pub mod hint;
pub mod input_group;
pub mod label;
pub mod list;
pub mod login_page;
pub mod modal;
pub mod nav;
pub mod page;
pub mod pagination;
pub mod panel;
pub mod popover;
pub mod select;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod table;
pub mod tabs;
#[deprecated(
    since = "0.4.0",
    note = "Use the `Content` component as a wrapper to standard HTML elements instead"
)]
pub mod text;
pub mod text_input_group;
pub mod title;
pub mod toast;
pub mod toolbar;
pub mod tooltip;

#[cfg(feature = "tree")]
pub mod tree;
