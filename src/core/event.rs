use crate::input::{ controller, keyboard, mouse, joy };
use crate::maths::Vector2D;

#[derive(Clone)]
pub enum Event {
    Quit,
    AppTerminating,
    AppLowMemory,
    AppWillEnterBackground,
    AppDidEnterBackground,
    AppWillEnterForeground,
    AppDidEnterForeground,
    Window {
        window_id: u32,
        window_event: WindowEvent
    },
    KeyDown {
        window_id: u32,
        keycode: Option<keyboard::KeyCode>,
        scancode: Option<keyboard::ScanCode>,
        key_mod: keyboard::Mod, repeat: bool
    },
    KeyUp {
        window_id: u32,
        keycode: Option<keyboard::KeyCode>,
        scancode: Option<keyboard::ScanCode>,
        key_mod: keyboard::Mod,
        repeat: bool
    },
    TextEditing {
        window_id: u32,
        text: String,
        start: i32,
        length: i32
    },
    TextInput {
        window_id: u32,
        text: String
    },
    MouseMotion {
        window_id: u32,
        which: u32,
        mouse_state: mouse::State,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32
    },
    MouseButtonDown {
        window_id: u32,
        which: u32,
        mouse_button: mouse::Button,
        clicks: u8,
        x: i32,
        y: i32
    },
    MouseButtonUp {
        window_id: u32,
        which: u32,
        mouse_button: mouse::Button,
        clicks: u8,
        x: i32,
        y: i32
    },
    MouseWheel {
        window_id: u32,
        which: u32,
        x: i32,
        y: i32,
        direction: mouse::WheelDirection
    },
    JoyAxisMotion {
        which: u32,
        axis_index: u8,
        value: i16
    },
    JoyBallMotion {
        which: u32,
        ball_index: u8,
        xrel: i16,
        yrel: i16
    },
    JoyHatMotion {
        which: u32,
        hat_index: u8,
        state: joy::HatState
    },
    JoyButtonDown {
        which: u32,
        button_index: u8
    },
    JoyButtonUp {
        which: u32,
        button_index: u8
    },
    JoyDeviceAdded {
        which: u32
    },
    JoyDeviceRemoved {
        which: u32
    },
    ControllerAxisMotion {
        which: u32,
        axis: controller::Axis,
        value: i16
    },
    ControllerButtonDown {
        which: u32,
        button: controller::Button
    },
    ControllerButtonUp {
        which: u32,
        button: controller::Button
    },
    ControllerDeviceAdded {
        which: u32
    },
    ControllerDeviceRemoved {
        which: u32
    },
    ControllerDeviceRemapped {
        which: u32
    },
    FingerDown {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerUp {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerMotion {
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    DollarGesture {
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32
    },
    DollarRecord {
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32
    },
    MultiGesture {
        touch_id: i64,
        d_theta: f32,
        d_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16
    },
    ClipboardUpdate,
    DropFile {
        window_id: u32,
        filename: String
    },
    DropText {
        window_id: u32,
        filename: String
    },
    DropBegin {
        window_id: u32
    },
    DropComplete {
        window_id: u32
    },
    AudioDeviceAdded {
        which: u32,
        is_capture: bool
    },
    AudioDeviceRemoved {
        which: u32,
        is_capture: bool
    },
    RenderTargetsReset,
    RenderDeviceReset,
    User {
        window_id: u32,
        kind: u32,
        code: i32,
        data1: *mut std::os::raw::c_void,
        data2: *mut std::os::raw::c_void
    },
    Unknown {
        kind: u32
    },
}

#[derive(Copy, Clone)]
pub enum WindowEvent {
    None,
    Shown,
    Hidden,
    Exposed,
    Moved(Vector2D),
    Resize(Vector2D),
    SizeChanged(Vector2D),
    Minimised,
    Maximised,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
    TakeFocus,
    HitTest,
}
