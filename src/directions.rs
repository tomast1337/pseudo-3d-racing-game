#[derive(PartialEq, Debug)]
pub enum MoveDirection {
    Forward,
    Break,
    Stopped,
}

#[derive(PartialEq, Debug)]
pub enum TrunDirection {
    Left,
    Right,
    None,
}