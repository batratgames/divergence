#[derive(Copy, Clone)]
pub enum Axis {
    LeftX,
    LeftY,
    RightX,
    RightY,
    TriggerLeft,
    TriggerRight,
}

#[derive(Copy, Clone)]
pub enum Button {
    A,
    B,
    X,
    Y,
    Back,
    Guide,
    Start,
    LeftStick,
    RightStick,
    LeftShoulder,
    RightShoulder,
    DPadUp,
    DPadDown,
    DPadLeft,
    DPadRight,
}
