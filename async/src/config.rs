use std::{collections::HashMap, fmt, path::PathBuf};

use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use serde_derive::Deserialize;

use crate::action::Action;

#[derive(Clone, Debug, Deserialize, Default)]
pub struct AppConfig {
  pub data_dir: PathBuf,
  pub config_dir: PathBuf,
  pub config: PathBuf,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Config {
  #[serde(default, flatten)]
  pub config: AppConfig,
  #[serde(default)]
  pub keymap: KeyMap,
}

#[derive(Clone, Debug, Default)]
pub struct KeyMap(pub HashMap<KeyEvent, Action>);

impl<'de> Deserialize<'de> for KeyMap {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    struct KeyMapVisitor;

    impl<'de> Visitor<'de> for KeyMapVisitor {
      type Value = KeyMap;

      fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a keymap with string representation of KeyEvent as key and Action as value")
      }

      fn visit_map<M>(self, mut access: M) -> Result<KeyMap, M::Error>
      where
        M: MapAccess<'de>,
      {
        let mut keymap = HashMap::new();

        // While there are entries in the map, read them
        while let Some((key_str, action)) = access.next_entry::<String, Action>()? {
          // Convert the string key back to a KeyEvent.
          let key_event = parse_key_event(&key_str).map_err(de::Error::custom)?;
          keymap.insert(key_event, action);
        }

        Ok(KeyMap(keymap))
      }
    }
    deserializer.deserialize_map(KeyMapVisitor)
  }
}

fn parse_key_event(raw: &str) -> Result<KeyEvent, String> {
  let raw_lower = raw.to_ascii_lowercase();
  let (remaining, modifiers) = extract_modifiers(&raw_lower);
  parse_key_code_with_modifiers(remaining, modifiers)
}

fn extract_modifiers(raw: &str) -> (&str, KeyModifiers) {
  let mut modifiers = KeyModifiers::empty();
  let mut current = raw;

  loop {
    match current {
      rest if rest.starts_with("ctrl-") => {
        modifiers.insert(KeyModifiers::CONTROL);
        current = &rest[5..];
      },
      rest if rest.starts_with("alt-") => {
        modifiers.insert(KeyModifiers::ALT);
        current = &rest[4..];
      },
      rest if rest.starts_with("shift-") => {
        modifiers.insert(KeyModifiers::SHIFT);
        current = &rest[6..];
      },
      _ => break, // break out of the loop if no known prefix is detected
    };
  }

  (current, modifiers)
}

fn parse_key_code_with_modifiers(raw: &str, mut modifiers: KeyModifiers) -> Result<KeyEvent, String> {
  let c = match raw {
    "esc" => KeyCode::Esc,
    "enter" => KeyCode::Enter,
    "left" => KeyCode::Left,
    "right" => KeyCode::Right,
    "up" => KeyCode::Up,
    "down" => KeyCode::Down,
    "home" => KeyCode::Home,
    "end" => KeyCode::End,
    "pageup" => KeyCode::PageUp,
    "pagedown" => KeyCode::PageDown,
    "backtab" => {
      modifiers.insert(KeyModifiers::SHIFT);
      KeyCode::BackTab
    },
    "backspace" => KeyCode::Backspace,
    "delete" => KeyCode::Delete,
    "insert" => KeyCode::Insert,
    "f1" => KeyCode::F(1),
    "f2" => KeyCode::F(2),
    "f3" => KeyCode::F(3),
    "f4" => KeyCode::F(4),
    "f5" => KeyCode::F(5),
    "f6" => KeyCode::F(6),
    "f7" => KeyCode::F(7),
    "f8" => KeyCode::F(8),
    "f9" => KeyCode::F(9),
    "f10" => KeyCode::F(10),
    "f11" => KeyCode::F(11),
    "f12" => KeyCode::F(12),
    "space" => KeyCode::Char(' '),
    "hyphen" => KeyCode::Char('-'),
    "minus" => KeyCode::Char('-'),
    "tab" => KeyCode::Tab,
    c if c.len() == 1 => {
      let mut c = c.chars().next().unwrap();
      if modifiers.contains(KeyModifiers::SHIFT) {
        c = c.to_ascii_uppercase();
      }
      KeyCode::Char(c)
    },
    _ => return Err(format!("Unable to parse {raw}")),
  };
  Ok(KeyEvent::new(c, modifiers))
}

impl Config {
  pub fn new() -> Result<Self, config::ConfigError> {
    let data_dir = crate::utils::get_data_dir().expect("Unable to get data directory");
    let config_dir = crate::utils::get_config_dir().expect("Unable to get config directory");
    let mut builder = config::Config::builder()
      .set_default("data_dir", data_dir.to_str().unwrap())?
      .set_default("config_dir", config_dir.to_str().unwrap())?
      .set_default("config", config_dir.join("config.toml").to_str().unwrap())?;

    builder = builder
      .add_source(config::File::from(config_dir.join("config.toml")).format(config::FileFormat::Toml).required(false));

    builder.build()?.try_deserialize()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_keys() {
    assert_eq!(parse_key_event("a").unwrap(), KeyEvent::new(KeyCode::Char('a'), KeyModifiers::empty()));

    assert_eq!(parse_key_event("enter").unwrap(), KeyEvent::new(KeyCode::Enter, KeyModifiers::empty()));

    assert_eq!(parse_key_event("esc").unwrap(), KeyEvent::new(KeyCode::Esc, KeyModifiers::empty()));
  }

  #[test]
  fn test_with_modifiers() {
    assert_eq!(parse_key_event("ctrl-a").unwrap(), KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL));

    assert_eq!(parse_key_event("alt-enter").unwrap(), KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT));

    assert_eq!(parse_key_event("shift-esc").unwrap(), KeyEvent::new(KeyCode::Esc, KeyModifiers::SHIFT));
  }

  #[test]
  fn test_multiple_modifiers() {
    assert_eq!(
      parse_key_event("ctrl-alt-a").unwrap(),
      KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL | KeyModifiers::ALT)
    );

    assert_eq!(
      parse_key_event("ctrl-shift-enter").unwrap(),
      KeyEvent::new(KeyCode::Enter, KeyModifiers::CONTROL | KeyModifiers::SHIFT)
    );
  }

  #[test]
  fn test_invalid_keys() {
    assert!(parse_key_event("invalid-key").is_err());
    assert!(parse_key_event("ctrl-invalid-key").is_err());
  }

  #[test]
  fn test_case_insensitivity() {
    assert_eq!(parse_key_event("CTRL-a").unwrap(), KeyEvent::new(KeyCode::Char('a'), KeyModifiers::CONTROL));

    assert_eq!(parse_key_event("AlT-eNtEr").unwrap(), KeyEvent::new(KeyCode::Enter, KeyModifiers::ALT));
  }
}
