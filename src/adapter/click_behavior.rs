use std::thread;
use std::time::Duration;
use enigo::Key;
use crate::common::input_enums::MouseClickType;

pub trait ClickBase {

}

pub trait KeyBoardClick<Type: ?Sized>: ClickBase {
    fn click(&mut self, input: &Type);
    fn click_hold(&mut self, input: &Type, time: Duration);
}

pub trait MouseClick<Type: ?Sized>: ClickBase {
    fn click(&mut self, input: &Type);
    fn click_hold(&mut self, input: &Type, time: Duration);
}