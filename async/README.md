# ratatui-async-template

<img width="757" alt="image" src="https://github.com/ratatui-org/ratatui-async-template/assets/1813121/f5c872fa-2c78-41af-82cd-717bdf7d0c1e">

### Features

- Uses [tokio](https://tokio.rs/) for async events
- Logs [tracing](https://github.com/tokio-rs/tracing)
- [better-panic](https://github.com/mitsuhiko/better-panic)
- [color-eyre](https://github.com/eyre/color-eyre)
- [human-panic](https://github.com/rust-cli/human-panic)
- Clap for command line argument parsing
- `Component` trait with
  [`App`](https://github.com/ratatui-org/ratatui-async-template/blog/main/src/components/app.rs)
  component as an example

### Usage

You can start by git cloning the project:

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/ratatui-org/ratatui-async-template --name ratatui-hello-world
$ cd ratatui-hello-world
$ cargo run # Press `q` to exit
```

### Documentation

Read documentation on design decisions in the template here: <https://ratatui-org.github.io/ratatui-async-template/>
