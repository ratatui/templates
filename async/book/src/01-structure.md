# `main.rs`

In this section, let's just cover the contents of `main.rs`, `build.rs` and `utils.rs`.

The `main.rs` file is the entry point of the application. Here's the complete `main.rs` file:

```rust,no_run,noplayground
{{#include ../../src/main.rs:all}}
```

We'll describe below in more detail the different aspects of this file.

````admonish
You'll notice that `main.rs` has `use ratatui_async_template::{...}`.

You will have to change `ratatui_async_template` what whatever you end up renaming your project.

The contents that are imported on this line are from the `lib.rs` file:

```rust,no_run,noplayground
{{#include ../../src/lib.rs}}
```

We will cover these modules in more detail in the next sections.

````

In essence, the `main` function creates an instance of `App` and calls `app.run()`, which runs the "get event -> update state -> render" loop.
We will talk more about this in a later section.

This `main.rs` file incorporates some key features that are not necessarily related to `ratatui`, but in my opinion, essential for any Terminal User Interface (TUI) program:

- Command Line Argument Parsing (`clap`)
- XDG Base Directory Specification
- Logging
- Panic Handler

These are described in more detail in the [`utils.rs` section](./10-structure.md).