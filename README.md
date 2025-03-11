# Ratatui Templates 🧀

This repository contains templates for bootstrapping a Rust
[TUI](https://en.wikipedia.org/wiki/Text-based_user_interface) application with
[`Ratatui`](https://github.com/ratatui/ratatui) and
[`crossterm`](https://github.com/crossterm-rs/crossterm).

## Getting Started

1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation)

   ```shell
   cargo install cargo-generate
   ```

2. Create a new app based on this repository:

   ```shell
   cargo generate ratatui/templates
   ```

3. Choose one of the following templates:

   - [Hello World](./hello-world/README.md): A "Hello, World!" example.
   - [Simple](./simple/README.md) | [Simple Async](./simple-async/README.md): A simple example.
   - [Event Driven](./event-driven/README.md) | [Event Driven Async](./event-driven-async/README.md): An example of an event-driven TUI application.
   - [Component](./component/README.md): An example of a component-based TUI application.

## Contributing

To keep the generated code up to date, install [`just`] and run `just generate-all` (or a specific
template - e.g. `just generate-simple`).

[`just`]: https://just.systems/
