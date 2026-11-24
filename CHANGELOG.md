# Changelog

## 0.9.0 (unreleased)

### Breaking changes

- Replaced `chrono` with Jiff 1.x. `CalendarMonthProperties` and
  `DatePickerProperties` now use `jiff::civil::Date` and `jiff::civil::Weekday`,
  including date callbacks. Use `Date::new(year, month, day)?` instead of
  `NaiveDate::from_ymd_opt(...)`, and `Weekday::Monday` instead of `Weekday::Mon`.
- `TimestampProperties::date` now accepts `Option<jiff::Timestamp>` instead of
  `Option<chrono::DateTime<chrono::Utc>>`. Parse RFC 3339 strings with
  `"2024-02-29T12:00:00Z".parse::<jiff::Timestamp>()?`, or use
  `jiff::Timestamp::now()` for the current instant.
- Dates are limited to Jiff's supported years (-9999 through 9999).
- The `localization` feature now uses the browser's locale and `Intl` Gregorian
  month/weekday formatting. Native rendering uses English labels. Removed
  `sys-locale`; browser builds do not bundle a time-zone database.
- Toast expiration now uses a monotonic clock. Newly added shorter timeouts are
  scheduled promptly, and long delays are capped and rescheduled safely.

### Other changes

- Added the Level layout as `LayoutLevel` and `LevelItem`, with optional gutter.
- Added the `Timestamp` component, with date/time formats, custom
  `Intl.DateTimeFormat` options, UTC display, and default or custom tooltips.
- Added the `Hero` component, with light/dark background images and gradients,
  configurable body width, and the (beta) glass variant.
- Avatar supports colorful avatars with `color` and `initials`.
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
