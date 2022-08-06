# Arcen

A native GUI framework for Neutron (ARC), based on Yew.

## Architecture

Like Yew, Arcen uses a declarative framework with `.rsx` files. Which are an extension to rust modules. Instead of being a normal module. It is parsed differently, namely its functions. At first, functions marked with `#[component]` are expanded to become structs. But later on, the functions marked with `#[component]` are actually treated as special VDOM elements. And like functions, called each time some state changes.

## Structure

Within `src/`, we have:

- `component`, reusable arcen components / widgets that make up the skeleton of an arcen app
- `animation`, a spline animation library for 2D splines and common animations like scale & rotation
- `renderer`, a swappable backend for drawing the VDOM. For native, creates command buffers and vertex specs and updates (uniform buffer and vertex buffer and depth and maybe stencil). For the web, converts VDOM to an actual DOM renderable directly by horizon / servo. The key thing is wasm and wgpu and the environment
- `parser`, the main parser for acx
