use crossterm_keybind::KeyBind;

// NOTE
// The KeyBindTrait will be impletemeted, so you can use the methods to generate and load a
// keybind config, and users can easily customize from a config.
// https://docs.rs/crossterm-keybind/latest/crossterm_keybind/trait.KeyBindTrait.html
// 
// Example using case: 
// use `to_toml_example(path)` to a file, after user customized it, it can loaded by
// `init_and_load(path)` when app starting.
#[derive(KeyBind)]
pub enum AppKeyEvent {
    /// The app will be closed with following key bindings
    /// - combin key Control and c
    /// - single key Esc
    /// - single key q
    #[keybindings["Control+c", "q", "Esc"]]
    Quit,
}
