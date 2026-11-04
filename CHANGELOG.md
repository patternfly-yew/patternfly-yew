# Changelog

## Unreleased

- Alert titles now render as `h4` headings by default, replacing `p`. Use
  `title_level` to select the heading level appropriate for your page.
- Alert titles accept rich `Html`; existing String values in Yew properties
  continue to work. Direct assignments to `AlertProperties.title` now require
  `.into()` when starting with a String.
- Alert and AlertGroup support opt-in polite live regions with `live_region`.
- Alert supports arbitrary `action_links` alongside existing callback `actions`.
  Use `AlertActionLink` with `href`, or supply a router link as its children with
  `pf-v6-c-button pf-m-link pf-m-inline` classes.
- Added expandable alerts, localized `variant_label` and `toggle_aria_label`,
  custom icons, and title truncation by line count with a positioned tooltip.
  The existing `truncate` flag continues to truncate to one line.
- Removed Alert's literal `aria_label` root attribute. The title's screen-reader
  prefix identifies the variant. Close buttons now derive their accessible name
  from “Close” and the title, including its variant prefix and rich content.
- Corrected Banner's screen-reader class to `pf-v6-screen-reader`. Banner already
  supports all v6 colors/statuses, sticky and pill styling. Supply
  `screen_reader_text` for status banners and use children for icons and layouts.
- Tooltip supports optional rich `content`, keyboard focus, and positioning.
