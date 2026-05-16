#[derive(PartialEq, Debug)]
pub enum MoveDirection {
    Forward,
    Brake,
    Coast,
}

#[derive(PartialEq, Debug)]
pub enum TurnDirection {
    Left,
    Right,
    None,
}
