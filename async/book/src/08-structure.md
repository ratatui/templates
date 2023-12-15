# `utils.rs`

### Command Line Argument Parsing (`clap`)

In this file, we define a [`clap`](https://docs.rs/clap/latest/clap/) `Args` struct.

```rust,no_run,noplayground
{{#include ../../ratatui-counter/src/cli.rs}}
```

This allows us to pass command line arguments to our terminal user interface if we need to.

![](https://user-images.githubusercontent.com/1813121/252718163-ab1945d1-7d44-4b5b-928d-1164ac99f2c9.png)

In addtion to command line arguments, we typically want the version of the command line program to
show up on request. In the `clap` command, we pass in an argument called `version()`. This
`version()` function (defined in `src/utils.rs`) uses a environment variable called
`RATATUI_ASYNC_TEMPLATE_GIT_INFO` to get the version number with the git commit hash.
`RATATUI_ASYNC_TEMPLATE_GIT_INFO` is populated in `./build.rs` when building with `cargo`, because
of this line:

```rust
  println!("cargo:rustc-env=RATATUI_ASYNC_TEMPLATE_GIT_INFO={}", git_describe);
```

![](https://user-images.githubusercontent.com/1813121/253160580-dc537c49-4191-4821-874a-9efc73cfe098.png)

You can configure what the version string should look like by modifying the string template code in
`utils::version()`.

### XDG Base Directory Specification

Most command line tools have configuration files or data files that they need to store somewhere. To
be a good citizen, you might want to consider following the XDG Base Directory Specification.

This template uses `directories-rs` and `ProjectDirs`'s config and data local directories. You can
find more information about the exact location for your operating system here:
<https://github.com/dirs-dev/directories-rs#projectdirs>.

This template also prints out the location when you pass in the `--version` command line argument.

![](https://user-images.githubusercontent.com/1813121/252721469-4d5ec38b-e868-46b4-b7b7-1c2c8bc496ac.png)

There are situations where you or your users may want to override where the configuration and data
files should be located. This can be accomplished by using the environment variables
`RATATUI_ASYNC_TEMPLATE_CONFIG` and `RATATUI_ASYNC_TEMPLATE_DATA`.

The functions that calculate the config and data directories are in `src/utils.rs`. Feel free to
modify the `utils::get_config_dir()` and `utils::get_data_dir()` functions as you see fit.

### Logging

The `utils::initialize_logging()` function is defined in `src/utils.rs`. The log level is decided by
the `RUST_LOG` environment variable (default = `log::LevelFilter::Info`). In addition, the location
of the log files are decided by the `RATATUI_ASYNC_TEMPLATE_DATA` environment variable (default =
`XDG_DATA_HOME (local)`).

I tend to use `.envrc` and `direnv` for development purposes, and I have the following in my
`.envrc`:

```bash
{{#include ../../ratatui-counter/.envrc}}
```

This puts the log files in the `RATATUI_ASYNC_TEMPLATE_DATA` folder, i.e. `.data` folder in the
current directory, and sets the log level to `RUST_LOG`, i.e. `debug` when I am prototyping and
developing using `cargo run`.

![Top half is a Iterm2 terminal with the TUI showing a Vertical split with `tui-logger` widget. Bottom half is a ITerm2 terminal showing the output of running `tail -f` on the log file.](https://user-images.githubusercontent.com/1813121/254093932-46d8c6fd-c572-4675-bcaf-45a36eed51ff.png)

Using the `RATATUI_ASYNC_TEMPLATE_CONFIG` environment variable also allows me to have configuration
data that I can use for testing when development that doesn't affect my local user configuration for
the same program.

### Panic Handler

Finally, let's discuss the `initialize_panic_handler()` function, which is also defined in
`src/utils.rs`, and is used to define a callback when the application panics. Your application may
panic for a number of reasons (e.g. when you call `.unwrap()` on a `None`). And when this happens,
you want to be a good citizen and:

1. provide a useful stacktrace so that they can report errors back to you.
2. not leave the users terminal state in a botched condition, resetting it back to the way it was.

In the screenshot below, I added a `None.unwrap()` into a function that is called on a keypress, so
that you can see what a prettier backtrace looks like:

![](https://user-images.githubusercontent.com/1813121/266889163-2fb6b210-58aa-4519-b491-0d35d9fa2c87.png)

`utils::initialize_panic_handler()` also calls `Tui::new().exit()` to reset the terminal state back
to the way it was before the user started the TUI program. We'll learn more about the `Tui` in the
next section.
