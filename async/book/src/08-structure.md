# `components/logger.rs`

Here's an example of a simpler `Logger` component that is used instead the `Home` component.
This `Logger` shows log output in the terminal user interface.
This uses `tracing` with the excellent [`tui-logger` crate](https://github.com/gin66/tui-logger).

```rust,no_run,noplayground
{{#include ../../src/components/logger.rs}}
```

![](https://user-images.githubusercontent.com/1813121/254452502-879beb8a-77dd-4475-bb55-1b15a443c747.gif)
