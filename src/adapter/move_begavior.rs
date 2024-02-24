
pub trait MoveBase {}

pub trait MouseMove: MoveBase {
    fn move_to(&self, x: i16, y: i16);
}