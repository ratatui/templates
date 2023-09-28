# `main.rs`

In this section, let's just cover the contents of `main.rs`, `build.rs` and `utils.rs`.

The `main.rs` file is the entry point of the application. Here's the complete `main.rs` file:

```rust,no_run,noplayground
{{#include ../../ratatui-counter/src/main.rs:all}}
```

We'll describe below in more detail the different aspects of this file.

In essence, the `main` function creates an instance of `Runner` and calls `runner.run()`, which runs
the "`handle event` -> `update state` -> `draw`" loop. We will talk more about this in a later
section.

This `main.rs` file incorporates some key features that are not necessarily related to `ratatui`,
but in my opinion, essential for any Terminal User Interface (TUI) program:

- Command Line Argument Parsing (`clap`)
- XDG Base Directory Specification
- Logging
- Panic Handler

These are described in more detail in the [`utils.rs` section](./08-structure.md).
