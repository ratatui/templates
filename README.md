# Ratatui Templates 🧀

This repository contains templates for bootstrapping a Rust
[TUI](https://en.wikipedia.org/wiki/Text-based_user_interface) application with
[`Ratatui`](https://github.com/ratatui-org/ratatui) &
[`crossterm`](https://github.com/crossterm-rs/crossterm).

## Getting Started

1. Install [`cargo-generate`](https://github.com/cargo-generate/cargo-generate#installation)

   ```shell
   cargo install cargo-generate
   ```

2. Create a new app based on this repository:

   ```shell
   cargo generate ratatui-org/ratatui-template
   ```

3. Choose one of the following templates:

   - [Simple](./simple/README.md)
   - [Simple Async](./simple-async/README.md)
   - [Component](./component/README.md)
