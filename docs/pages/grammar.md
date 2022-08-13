---
layout: default
title: Grammar
---

## Spec

```rust
rx: "rx" ~ rx_scope

rx_scope: "{" ~ expr ~ "}"

expr: ident ~ params? ~ "{" ~ expr ~ "}" | expr

params: "(" ~ param_expr ~ ")"

param_expr: ident ~ "=" ~ attr

attr: rust_var | literal | rust_expr
```
