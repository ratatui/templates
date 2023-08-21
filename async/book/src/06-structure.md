# `components/mod.rs`

In `components/mod.rs`, we implement a `trait` called `Component`:

```rust,no_run,noplayground
{{#include ../../src/components/mod.rs:component}}
```

I personally like keeping the functions for `handle_events` (i.e. event -> action mapping), `dispatch` (i.e. action -> state update mapping) and `render` (i.e. state -> drawing mapping) all in one file for each component of my application.

There's also an `init` function that can be used to setup the `Component` when it is loaded.

The `Home` struct (i.e. the root struct that may hold other `Component`s) will implement the `Component` trait.
We'll have a look at `Home` next.
