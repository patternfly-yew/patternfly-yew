// Entry point of the bundle used by the `charts` feature of patternfly-yew.
//
// It exposes a minimal API to create and render React elements of the PatternFly charts
// (which are built on top of Victory) from Rust.
//
// Rebuild the bundle (`src/charts/js/patternfly-charts.js`) with: `npm ci && npm run build`

import { createElement } from "react";
import { flushSync } from "react-dom";
import { createRoot } from "react-dom/client";
import * as Charts from "@patternfly/react-charts/victory";

const containers = new Map();

/// Resolve a component type by its exported name (e.g. `ChartBar`).
function resolve(type) {
  if (typeof type !== "string") {
    return type;
  }
  const component = Charts[type];
  if (component === undefined) {
    throw new Error(`Unknown PatternFly chart component: ${type}`);
  }
  return component;
}

export function create_root(container) {
  return createRoot(container);
}

export function render_root(root, element) {
  // Render synchronously, so that the committed tree never references callbacks which have
  // already been released on the Rust side.
  flushSync(() => root.render(element));
}

export function unmount_root(root) {
  root.unmount();
}

/// Create a container type combining two behaviors (e.g. `voronoi` and `cursor`).
///
/// The type is cached, as React requires a stable component type across renders.
export function container_type(first, second) {
  const key = `${first}+${second}`;
  let container = containers.get(key);
  if (container === undefined) {
    container = Charts.createContainer(first, second);
    containers.set(key, container);
  }
  return container;
}

export function create_element(type, props, children) {
  return createElement(resolve(type), props, ...children);
}
