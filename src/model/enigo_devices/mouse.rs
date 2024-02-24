use std::thread;
use std::time::Duration;
use enigo::{Enigo, MouseButton, MouseControllable};
use crate::adapter::click_behavior::{ClickBase, MouseClick};

#[derive(Debug)]
pub struct Mouse {
    engio: Enigo
}

impl Mouse {
    fn new(engio: Enigo) -> Mouse {
        Mouse { engio }
    }
}

impl ClickBase for Mouse {}

impl MouseClick<MouseButton> for Mouse {
    fn click(&mut self, input: &MouseButton) {
        self.engio.mouse_click(*input);
    }

    fn click_hold(&mut self, input: &MouseButton, time: Duration) {
        self.engio.mouse_down(*input);
        thread::sleep(time);
        self.engio.mouse_up(*input);
    }
}