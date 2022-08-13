---
layout: default
title: Grammar
---

## Spec

```rust
xml: tag

tag: "<" ~ tag_params ~ ">" | tag_one_line
tag_one_line: "<" ~ tag_params ~ "/>"
tag_end: "</" ~ tag_name ~ ">"
tag_params: tag_name ~ tag_attribute*
tag_attribute: key ~ ("=" ~ attribute_val)?
attribute_val: "{" ~ expr ~ "}"

tag_content: text | expr

text: "\w+"
```
