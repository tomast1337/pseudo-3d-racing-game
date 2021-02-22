#[derive(PartialEq, Debug)]
pub enum MoveDirection {
    Forward,
    Break,
}

#[derive(PartialEq, Debug)]
pub enum TrunDirection {
    Left,
    Right,
    None,
}