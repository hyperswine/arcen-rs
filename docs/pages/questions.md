---
layout: default
title: Questions
---

## Questions for design

let val be the answer of the expression
an arg's value should match its type. Which is usually a rust like type, either a single var, tuple or array
it can be an rx expression, which is pretty much a literal, identifier or string
or it can be a rust expression, which would highlighted by braces {}
e.g. Box(height="2rem"), Box(h=2) are valid forms
Box(height={var}) means you want to use a rust variable
