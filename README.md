# `template` 🧀

A template for bootstrapping a Rust
[**TUI**](https://en.wikipedia.org/wiki/Text-based_user_interface) application with
[`Ratatui`](https://github.com/ratatui-org/ratatui) &
[`crossterm`](https://github.com/crossterm-rs/crossterm).

<img src="https://raw.githubusercontent.com/ratatui-org/ratatui/b33c878808c4c40591d7a2d9f9d94d6fee95a96f/examples/demo2.gif" width="600">

## Project structure

```
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

```sh
cargo install cargo-generate
```

2. Clone this repository via `cargo-generate`:

```sh
cargo generate --git https://github.com/ratatui-org/template --name <project-name>
```

## See also

- [Rust Munich Meetup #8 - Designing TUI Applications in Rust](https://www.youtube.com/watch?v=ogdJnOLo238)
  (2021/outdated)
