# Ratatui Simple Template

A simple example of a Ratatui application.

## Design choices

We have a small `App` struct that has a main loop that calls methods to handle events and draw the
UI. The app can be quit by pressing any of Q/Esc/Ctrl+C.

We use [color-eyre](https://docs.rs/color-eyre) for simplifying any errors that need to be reported
to the console.
