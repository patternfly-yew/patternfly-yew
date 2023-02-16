# Icons generator

Generate Icons from the set of icons listed on the PatterFly website.

Source: https://github.com/patternfly/patternfly-org/blob/main/packages/v4/patternfly-docs/content/design-guidelines/styles/icons/icons.js

## Update

* Fetch a new version of the description

  ```shell
  curl -sSL https://raw.githubusercontent.com/patternfly/patternfly-org/main/packages/v4/patternfly-docs/content/design-guidelines/styles/icons/icons.js -o icons.mjs
  ```

* Convert file

  ```shell
  node ./generate.mjs > ../../src/icon/generated.rs
  ```