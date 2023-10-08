# ratatui-async-template

![out](https://github.com/ratatui-org/ratatui-async-template/assets/1813121/057a0fe9-9f6d-4f8c-963c-ca2725721bdd)

### Features

- Uses [tokio](https://tokio.rs/) for async events
- Logs [tracing](https://github.com/tokio-rs/tracing)
- [better-panic](https://github.com/mitsuhiko/better-panic)
- [color-eyre](https://github.com/eyre/color-eyre)
- [human-panic](https://github.com/rust-cli/human-panic)
- Clap for command line argument parsing
- `Component` trait with a
  [`Home`](https://github.com/ratatui-org/ratatui-async-template/blob/main/template/src/components/home.rs)
  component as an example

### Usage

You can start by git cloning the project:

```bash
$ cargo install cargo-generate
$ cargo generate --git https://github.com/ratatui-org/ratatui-async-template --name ratatui-hello-world
$ cd ratatui-hello-world
```

You can also use a
[`template.toml`](https://github.com/ratatui-org/ratatui-async-template/blob/main/.github/workflows/template.toml)
file to skip the prompts:

```bash
$ cargo generate --git https://github.com/ratatui-org/ratatui-async-template --template-values-file .github/workflows/template.toml --name ratatui-hello-world
# OR generate from local clone
$ cargo generate --path . --template-values-file .github/workflows/template.toml --name ratatui-hello-world
```

**Run**

```bash
$ cargo run # Press `q` to exit
```

**Help**

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

**Version**

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
direnv: loading ~/gitrepos/ratatui-async-template/ratatui-hello-world/.envrc
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

Config directory: /Users/kd/gitrepos/ratatui-async-template/ratatui-hello-world/.config
Data directory: /Users/kd/gitrepos/ratatui-async-template/ratatui-hello-world/.data
```

### Documentation

Read documentation on design decisions in the template here:
<https://ratatui-org.github.io/ratatui-async-template/>

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
