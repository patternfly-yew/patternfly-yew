# Migrating from 0.4 to 0.5

## PatternFly v5

This version is based on PatternFly v5. This requires a change of the PatternFly assets, probably an your `package.json`
file.

## Next versions & Deprecations

All features deprecated in 0.4 have been removed. This includes:

* `AppLauncher` – Deprecated by PatternFly, replaced by new menu
* `ContextSelector` – Deprecated by PatternFly, replaced by new menu

All `next` variants have replaced their original implementations.  This includes:

* `Card`
* `FormSelect`
* `Table`
* `TextArea`
* `TextInput`
* `Tree`
* `Toolbar`

## Function Components

A lot of components have been migrated to function based components. Which in some cases required a breaking change.

In most cases, this means that components no longer manage their state internally, and a combination of callbacks
and hooks should be used in order to manage the state outside the component.

While this requires a change to when upgrading, it also solves a bunch of "initial data" issues, where the properties
would override a state after rendering.

### TextArea & TextInput

If you want to track the state of the entered data, a minimal example now looks like this:

```rust
use yew::prelude::*;
use patternfly_yew::prelude::*;

#[function_component(Example)]
fn example() -> Html {
  let value = use_state_eq(String::default);
  let onchange = {
     let value = value.clone();
     Callback::from(move |data| value.set(data))
  };

  html!(<TextInput value={(*value).clone()}/>)
}
```

## Table (& Tree) models

Both the table and tree have seen a major overhaul of their model and rendering traits.

The two bigger changes are that you now need to use a concrete type to identify columns and that it is now possible to
externalize the expandable state from the data.

The former change allows one to provide a concrete type (like an enum) to identify columns. This will be patched through
to the rendering trait, and so you can use a proper match with enums to render the content. Eliminating the `_` match,
which had to resolve to `html!()` for "anything else". You can still use `usize` as column key, but it is recommended
to define an enum. While this adds another type argument, this fixes the issue that adding new columns misaligned with
the rendering implementation.

Externalizing the expansion state from the actual data should improve the performance on bigger data sets. As you can
now use an `Rc`ed data set, without the need to convert everything to an alternate data model, storing the expansion
state in the process.

## Additional changes

### Components

#### About

- The `About` component has been renamed to `AboutModal`
- `brand_src` has been renamed to `brand_image_src` and is now required.
- `brand_alt` has been renamed to `brand_image_alt` and is now required.
- `children` is now required.
- `title` has been renamed to `product_name`, however if it is not used `aria-label` should be used in it place.
- `strapline` has been renamed to `trademark` and is now required.
- `hero_style` attribute has been removed, if you were using this to set a responsive background the following code is suggested:
    ```
    <style>
        {r#"
            @media only screen and (min-width: 576px) {
                .pf-v6-c-about-modal-box[style]  {
                    --pf-v6-c-about-modal-box--BackgroundImage: url("https://www.patternfly.org/assets/images/pfbg_992.jpg") !important;
                }
            }
            @media only screen and (min-width: 992px) {
                .pf-v6-c-about-modal-box[style]  {
                    --pf-v6-c-about-modal-box--BackgroundImage: url("https://www.patternfly.org/assets/images/pfbg_992@2x.jpg") !important;
                }
            }
        "#}
    </style>
  ```
  The position of the background has been changed, so consideration should be given to overriding these styles as well.


#### Alert component

- `AlertType::Default` has been renamed to `AlertType::Custom`

#### Avatar

- The `alt` attribute is now required. Previously it defaulted to :`"Avatar image"`

### Layouts

#### Flex

`FlexModifier::Default` & `FlexModifier::None` have been removed as they were not in the PatternFly 5 spec and should not have had an effect.
