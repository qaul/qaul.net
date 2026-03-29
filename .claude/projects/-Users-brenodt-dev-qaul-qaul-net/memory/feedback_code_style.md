---
name: Code style preferences
description: User preferences for readability — named booleans over long conditions, no long ternaries, prefer if/else for complex logic
type: feedback
---

- Extract long boolean conditions into named local variables that describe intent (e.g. `isPartialUpdate`, `isOpenRoom`), so conditionals read as plain English.
- Avoid long/multi-line ternaries. Ternaries only add value when they're simpler to read than a traditional if/else.
- Prefer idiomatic Flutter patterns like `copyWith` over verbose constructor calls when updating a few fields.

**Why:** Readability and clarity over brevity. The user values code that reads like prose at the control-flow level.

**How to apply:** When writing or reviewing Dart/Flutter code, default to named booleans + if/else for anything non-trivial. Use ternaries only for simple, single-line expressions.
