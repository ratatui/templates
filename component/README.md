# async-template

![async template demo](https://user-images.githubusercontent.com/1813121/277114001-0d25a09c-f24e-4ffc-8763-cd258828cec0.gif)

## Usage

You can start by using `cargo-generate`:

```shell
cargo install cargo-generate
cargo generate ratatui/templates component --name ratatui-hello-world
cd ratatui-hello-world
```

## Features

- Uses [tokio](https://tokio.rs/) for async events
  - Start and stop key events to shell out to another TUI like vim
  - Supports suspend signal hooks
- Logs using [tracing](https://github.com/tokio-rs/tracing)
- [better-panic](https://github.com/mitsuhiko/better-panic)
- [color-eyre](https://github.com/eyre-rs/color-eyre)
- [human-panic](https://github.com/rust-cli/human-panic)
- Clap for command line argument parsing
- `Component` trait with
  [`Home`](https://github.com/ratatui/async-template/blob/main/template/src/components/home.rs)
  and
  [`Fps`](https://github.com/ratatui/async-template/blob/main/template/src/components/fps.rs)
  components as examples

## Advanced Usage

You can also use a
[`template.toml`](https://github.com/ratatui/async-template/blob/main/.github/workflows/template.toml)
file to skip the prompts:

```bash
$ cargo generate --git https://github.com/ratatui/async-template --template-values-file .github/workflows/template.toml --name ratatui-hello-world
# OR generate from local clone
$ cargo generate --path . --template-values-file .github/workflows/template.toml --name ratatui-hello-world
```

## Running your App

```bash
cargo run # Press `q` to exit
```

## Show help

```bash
$ cargo run -- --help
Hello World project using ratatui-template

Usage: ratatui-hello-world [OPTIONS]

Options:
  -t, --tick-rate <FLOAT>   Tick rate, i.e. number of ticks per second [default: 1]
  -f, --frame-rate <FLOAT>  Frame rate, i.e. number of frames per second [default: 60]
  -h, --help                Print help
  -V, --version             Print version
```

## Show `version`

Without direnv variables:

```bash
$ cargo run -- --version
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/ratatui-hello-world --version`
ratatui-hello-world v0.1.0-47-eb0a31a

Authors: Dheepak Krishnamurthy

Config directory: /Users/kd/Library/Application Support/com.kdheepak.ratatui-hello-world
Data directory: /Users/kd/Library/Application Support/com.kdheepak.ratatui-hello-world
```

With direnv variables:

```bash
$ direnv allow
direnv: loading ~/gitrepos/async-template/ratatui-hello-world/.envrc
direnv: export +RATATUI_HELLO_WORLD_CONFIG +RATATUI_HELLO_WORLD_DATA +RATATUI_HELLO_WORLD_LOG_LEVEL

$ # OR

$ export RATATUI_HELLO_WORLD_CONFIG=`pwd`/.config
$ export RATATUI_HELLO_WORLD_DATA=`pwd`/.data
$ export RATATUI_HELLO_WORLD_LOG_LEVEL=debug

$ cargo run -- --version
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/ratatui-hello-world --version`
ratatui-hello-world v0.1.0-47-eb0a31a

Authors: Dheepak Krishnamurthy

Config directory: /Users/kd/gitrepos/async-template/ratatui-hello-world/.config
Data directory: /Users/kd/gitrepos/async-template/ratatui-hello-world/.data
```

## Documentation

Read documentation on design decisions in the template here:
<https://ratatui.github.io/async-template/>

## Counter + Text Input Demo

This repo contains a `ratatui-counter` folder that is a working demo as an example. If you wish to
run a demo without using `cargo generate`, you can run the counter + text input demo by following
the instructions below:

```bash
git clone https://github.com/ratatui/async-template
cd async-template
cd ratatui-counter # counter + text input demo

export RATATUI_COUNTER_CONFIG=`pwd`/.config
export RATATUI_COUNTER_DATA=`pwd`/.data
export RATATUI_COUNTER_LOG_LEVEL=debug
# OR
direnv allow

cargo run
```

You should see a demo like this:

![counter demo](https://github.com/ratatui/async-template/assets/1813121/057a0fe9-9f6d-4f8c-963c-ca2725721bdd)
