use std::thread;
use std::time::Duration;
use enigo::{Enigo, Key, KeyboardControllable};
use crate::adapter::click_behavior::{ClickBase, KeyBoardClick};

#[derive(Debug)]
pub struct KeyBoard {
    enigo: Enigo
}

impl KeyBoard {
    pub(crate) fn new() -> KeyBoard {
        let enigo = Enigo::new();
        KeyBoard { enigo }
    }
}

impl ClickBase for KeyBoard {}

impl KeyBoardClick<str> for KeyBoard {
    fn click(&mut self, input: &str) {
        self.enigo.key_sequence_parse(input);
    }

    fn click_hold(&mut self, input: &str, time: Duration) {
        let key = Key::Layout(input.chars().last().unwrap());
        self.enigo.key_down(key);
        thread::sleep(time);
        self.enigo.key_up(key);
    }
}

impl KeyBoardClick<Key> for KeyBoard {
    fn click(&mut self, input: &Key) {
        self.enigo.key_click(*input);
    }

    fn click_hold(&mut self, input: &Key, time: Duration) {
        self.enigo.key_down(*input);
        thread::sleep(time);
        self.enigo.key_up(*input);
    }
}
