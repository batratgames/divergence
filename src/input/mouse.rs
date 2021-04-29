#[derive(Copy, Clone)]
pub enum Button {
    Unknown,
    Left,
    Middle,
    Right,
    X1,
    X2,
}

#[derive(Copy, Clone)]
pub enum WheelDirection {
    Normal,
    Flipped,
    Unknown(u32),
}
