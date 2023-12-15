# Ratatui templates 🧀

This repository contains templates for bootstrapping a Rust
[**TUI**](https://en.wikipedia.org/wiki/Text-based_user_interface) application with
[`Ratatui`](https://github.com/ratatui-org/ratatui) &
[`crossterm`](https://github.com/crossterm-rs/crossterm).

## Project structure

```text
src/
├── app.rs     -> holds the state and application logic
├── event.rs   -> handles the terminal events (key press, mouse click, resize, etc.)
├── handler.rs -> handles the key press events and updates the application
├── lib.rs     -> module definitions
├── main.rs    -> entry-point
├── tui.rs     -> initializes/exits the terminal interface
└── ui.rs      -> renders the widgets / UI
```

## Creating a project

1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation)

   ```shell
   cargo install cargo-generate
   ```

2. Create a new app based on this repository:

   ```shell
   cargo generate ratatui-org/ratatui-template simple
   ```

## See also

- [Rust Munich Meetup #8 - Designing TUI Applications in Rust](https://www.youtube.com/watch?v=ogdJnOLo238)
  (2021/outdated)
