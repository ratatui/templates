# ratatui-async-template

![out](https://github.com/ratatui-org/ratatui-async-template/assets/1813121/057a0fe9-9f6d-4f8c-963c-ca2725721bdd)

### Features

- Uses [tokio](https://tokio.rs/) for async events
- Logs [tracing](https://github.com/tokio-rs/tracing)
- [better-panic](https://github.com/mitsuhiko/better-panic)
- [color-eyre](https://github.com/eyre/color-eyre)
- [human-panic](https://github.com/rust-cli/human-panic)
- Clap for command line argument parsing
- `Component` trait with a [`Home`](https://github.com/ratatui-org/ratatui-async-template/blob/main/template/src/components/home.rs) component as an example

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

### Demo

Run the demo

```bash
cd ratatui-counter

export RATATUI_COUNTER_CONFIG=`pwd`/.config
export RATATUI_COUNTER_DATA=`pwd`/.data
export RATATUI_COUNTER_LOG_LEVEL=debug
# OR
direnv allow

cargo run
```
