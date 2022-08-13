---
layout: default
title: Examples
---

## RX

Here are some examples in action:

```rust
// rx_parser: cant detect an expression.. assuming its rust. Leaving it there (not expanding it in place in the output string)
let width = 2;

let rx = @{
    List {
        Box(padding="2rem", width={width}) {},
        Box {},
    },
};
```

RX auto parses your `.rx` and finds the local scope to match vars and generate `Node`s from. The result is code that is parseable by rustc. By default, `rx` is parsed at build time (in build.rs). To use arcen as a library, put:

```rust
// build.rs
fn main() {
    arcen::parse_rx()
}
```

Arcen provides an `arcen` language server for syntax highlighting of `.rx` files and rustc like pre-parsing of most errors. It actually incrementally parses your rx and calls rust-analyzer in the background. Which means you'll know whether the code really does work as you are writing the rx.
