This list is a collection of breaking changes:

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
                .pf-v5-c-about-modal-box[style]  {
                    --pf-v5-c-about-modal-box--BackgroundImage: url("https://www.patternfly.org/assets/images/pfbg_992.jpg") !important;
                }
            }
            @media only screen and (min-width: 992px) {
                .pf-v5-c-about-modal-box[style]  {
                    --pf-v5-c-about-modal-box--BackgroundImage: url("https://www.patternfly.org/assets/images/pfbg_992@2x.jpg") !important;
                }
            }
        "#}
    </style>
  ```
  The position of the background has been changed, so consideration should be given to overriding these styles as well.

#### Alert component
- `AlertType::Default` has been renamed to `AlertType::Custom`

#### AppLauncher
The AppLauncher component is now depreciated due to its depreciation in PatternFly 5. - https://patternfly-react-v5.surge.sh/components/menus/application-launcher

#### Avatar
- The `alt` attribute is now required. Previously it defaulted to :`"Avatar image"`

#### Card
The previous implementation has been deprecated for a new implementation.

#### ContentSelector
The ContentSelector component is now depreciated due to its depreciation in PatternFly 5. - https://pf5.patternfly.org/components/menus/context-selector/

#### FormSelect
The previous implementation has been deprecated for a new implementation.

#### Table
The previous implementation has been deprecated for a new implementation.

#### TextArea
The previous implementation has been deprecated for a new implementation.

#### TextInput
The previous implementation has been deprecated for a new implementation.

#### Toolbar
The previous implementation has been deprecated for a new implementation.

### Layouts
#### Flex
 `FlexModifier::Default` & `FlexModifier::None` have been removed as they were not in the PatternFly 5 spec and should not have had an effect.