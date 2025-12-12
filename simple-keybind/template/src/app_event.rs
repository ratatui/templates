use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum AppKeyEvent {
    /// The app will be closed with following key bindings
    /// - combin key Control and c
    /// - single key Esc
    /// - single key q
    #[keybindings["Control+c", "q", "Esc"]]
    Quit,
}
