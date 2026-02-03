import {iconsData} from "./icons.mjs";
import {iconsData as manualIconsData} from "./icons.manual.mjs";

class Generator {
  #known;
  #define = "";
  #impl = "";

  constructor() {
    this.#known = new Set();
  }

  #icon(icon) {
    if (icon.Name === undefined ) {
      return;
    }

    if (this.#known.has(icon.React_name)) {
      return;
    }
    this.#known.add(icon.React_name);

    let className = icon.Name;

    // fix up based on style

    switch (icon.Style) {
      case "fas":
        // as is
        break;
      case "fab":
        icon.Feature = "icons-fab";
        break;
      case "far":
        icon.Feature = "icons-far";
        break;
      case "":
        icon.Style = "plain";
        break;
      case "pficon":
        icon.Style = "pf";
        className = `pf-v6-${className}`;
        break;
      default:
        // This means we need to adap the generator
        throw "Unknown icon type: " + icon.Style;
    }

    // sanitize name

    let name = icon.React_name;
    if (name.endsWith("Icon")) {
      name = name.slice(0, -("Icon".length));
    }
    if (name.startsWith("Pficon")) {
      name = name.substring("Pficon".length);
    }

    // write

    let feature = "";
    if (icon.Feature !== undefined) {
      feature = `#[cfg(feature="${icon.Feature}")]
`;
    }

    this.#define += `
    /// ${icon.Contextual_usage}
    ${feature}${name},`;

    this.#impl += `
            ${feature}Self::${name} => classes.extend(super::${icon.Style}("${className}")),`;
  }

  add(icons) {
    for (const icon of icons) {
      if (Array.isArray(icon)) {
        this.add(icon);
      } else {
        this.#icon(icon);
      }
    }
    return this;
  }

  write() {

    console.log(`#[derive(Copy, Clone, Debug, PartialEq, Eq, strum_macros::EnumIter, strum_macros::EnumMessage, strum_macros::AsRefStr, strum_macros::IntoStaticStr)]
pub enum Icon {
    ${this.#define}
}
`);

    console.log(`
impl crate::core::AsClasses for Icon {
    fn extend_classes(&self, classes: &mut yew::prelude::Classes) {
        match self {
            ${this.#impl}
        }
    }
}
`);
  }

}

new Generator()
    .add(iconsData)
    .add(manualIconsData)
    .write()
;

