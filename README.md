# rust-tui-template

A template for bootstrapping a Rust [**TUI**](https://en.wikipedia.org/wiki/Text-based_user_interface) application with [`ratatui`](https://github.com/tui-rs-revival/ratatui) & [`crossterm`](https://github.com/crossterm-rs/crossterm).

<img src="https://raw.githubusercontent.com/tui-rs-revival/ratatui/master/assets/demo.gif" width="600">

### ratatui

> The library is based on the principle of immediate rendering with intermediate buffers. This means that at each new frame you should build all widgets that are supposed to be part of the UI. While providing a great flexibility for rich and interactive UI, this may introduce overhead for highly dynamic content. So, the implementation try to minimize the number of ansi escapes sequences generated to draw the updated UI. In practice, given the speed of Rust the overhead rather comes from the terminal emulator than the library itself.

> ratatui is an actively maintained fork of [tui-rs](https://github.com/fdehau/tui-rs).

#### [Documentation](https://docs.rs/tui)

### crossterm

> Crossterm is a pure-rust, terminal manipulation library that makes it possible to write cross-platform text-based interfaces (see [features](https://github.com/crossterm-rs/crossterm/blob/master/README.md#features)). It supports all UNIX and Windows terminals down to Windows 7 (not all terminals are tested,
> see [Tested Terminals](https://github.com/crossterm-rs/crossterm/blob/master/README.md#tested-terminals) for more info).

#### [Documentation](https://docs.rs/crossterm)

## Structure

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

## Usage

1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation)

```sh
cargo install cargo-generate
```

2. Clone this repository via `cargo-generate`:

```sh
cargo generate --git https://github.com/tui-rs-revival/rust-tui-template --name <project-name>
```

## See also

- [Rust Munich Meetup #8 - Designing TUI Applications in Rust](https://www.youtube.com/watch?v=ogdJnOLo238)
