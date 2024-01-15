# Icons generator

Generate Icons from the set of icons listed on the PatterFly website.

Source: https://raw.githubusercontent.com/patternfly/patternfly-org/v5/packages/documentation-site/patternfly-docs/content/design-guidelines/styles/icons/icons.js

## Update

* Fetch a new version of the description

  ```shell
  curl -sSL https://raw.githubusercontent.com/patternfly/patternfly-org/v5/packages/documentation-site/patternfly-docs/content/design-guidelines/styles/icons/icons.js -o icons.mjs
  ```

* Convert file

  ```shell
  node ./generate.mjs > ../../src/icon/generated.rs
  ```

* `rustfmt` the file

  ```shell
  rustfmt ../../src/icon/generated.rs
  ```