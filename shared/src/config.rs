use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::event::KeyboardEvent;

#[derive(Clone, Deserialize, Serialize)]
pub struct ViewerConfig {
    #[serde(default = "default_keymap")]
    pub keymap: HashMap<KeyboardEvent, Vec<String>>,
}

fn default_keymap() -> HashMap<KeyboardEvent, Vec<String>> {
    [
        (KeyboardEvent::NextImage, vec!["KeyD", "KeyL", "ArrowRight"]),
        (KeyboardEvent::PrevImage, vec!["KeyA", "KeyH", "ArrowLeft"]),
        (
            KeyboardEvent::NextDirectory,
            vec!["KeyS", "KeyJ", "ArrowDown"],
        ),
        (
            KeyboardEvent::PrevDirectory,
            vec!["KeyW", "KeyK", "ArrowUp"],
        ),
    ]
    .iter()
    .map(|(k, e)| (k.clone(), e.iter().map(|s| s.to_string()).collect()))
    .collect()
}

impl Default for ViewerConfig {
    fn default() -> Self {
        Self {
            keymap: default_keymap(),
        }
    }
}
