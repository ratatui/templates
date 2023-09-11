# ratatui-async-template

<img width="600" alt="image" src="https://github.com/ratatui-org/ratatui-async-template/assets/1813121/61d9f3a4-14d7-4bb8-85be-771fd5da4c0f">

### Features

- Uses [tokio](https://tokio.rs/) for async events
- Logs [tracing](https://github.com/tokio-rs/tracing)
- [better-panic](https://github.com/mitsuhiko/better-panic)
- [color-eyre](https://github.com/eyre/color-eyre)
- [human-panic](https://github.com/rust-cli/human-panic)
- Clap for command line argument parsing
- App with `Component` trait, with
  [`App`](https://github.com/ratatui-org/ratatui-async-template/blog/main/src/components/app.rs) and
  component as an example

### Usage

- Clone the repository
- Rename `ratatui-async-template` and `RATATUI_ASYNC_TEMPLATE` with your `app-name` and `APP_NAME`
  in the following files:
  - `build.rs`
  - `Cargo.toml`
  - `README.md`
  - `.github/workflows/cd.yml`
  - `.envrc`
