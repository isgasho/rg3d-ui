use crate::{
    core::{
        math::vec2::Vec2,
        pool::Handle,
    },
    UINode,
};
use std::any::Any;

pub enum UIEventKind {
    /// Generated when some mouse button was pressed.
    MouseDown {
        pos: Vec2,
        button: MouseButton,
    },

    /// Generated when some mouse button was released.
    MouseUp {
        pos: Vec2,
        button: MouseButton,
    },

    /// Generated when mouse cursor was moved in bounds of widget.
    MouseMove {
        pos: Vec2
    },

    /// Generated when some text was entered.
    Text {
        symbol: char
    },

    /// Generated when some key was pressed.
    KeyDown {
        code: KeyCode
    },

    /// Generated when some key was released.
    KeyUp {
        code: KeyCode
    },

    /// Generated when mouse wheel was rolled while cursor was in bounds of widget.
    MouseWheel {
        pos: Vec2,
        amount: f32,
    },

    /// Generated once when mouse leaves bounds of widget.
    MouseLeave,

    /// Generated once when mouse enters bounds of widget.
    MouseEnter,

    /// Generated by clickable widgets such as buttons.
    ///
    /// # Notes
    /// This event differs from [`MouseDown`] event! [`Click`] event will be generated only
    /// if button (or any other "clickable" widget) was previously pressed and mouse button
    /// was released right inside widget bounds.
    Click,

    /// Generated by widgets that has some numeric value that can change.
    NumericValueChanged {
        old_value: f32,
        new_value: f32,
    },

    /// Generated by any widget that has max numeric value (scroll bar for example).
    MaxValueChanged(f32),

    /// Generated by any widget that has min numeric value (scroll bar for example).
    MinValueChanged(f32),

    /// Generated by any ItemsControl that has selection behaviour.
    SelectionChanged(Option<usize>),

    /// Generated by opened window.
    Opened,

    /// Generated by closed window.
    Closed,

    /// Widget just got keyboard focus.
    GotFocus,

    /// Widget lost keyboard focus.
    LostFocus,

    /// Generated by window that has become minimized.
    Minimized(bool),

    /// Generated by window that has changed its ability to minimize.
    CanMinimizeChanged(bool),

    /// Generated by window that has changed its ability to close.
    CanCloseChanged(bool),

    /// Generated by checkbox that has changed its checked state.
    Checked(Option<bool>),

    /// Any kind of user-defined event.
    User(Box<dyn Any>),
}

/// Event is basic communication element that is used to deliver information to UI nodes
/// or some other places.
pub struct UIEvent {
    /// Flag which allows to mark event as handled. This can be useful if multiple listeners
    /// can handle event but event should be handled only once.
    ///
    /// # Notes
    ///
    /// This value does not have effect on event dispatcher.
    pub handled: bool,

    pub kind: UIEventKind,

    /// Handle of node for which this event was produced. Can be NONE if target is undefined,
    /// this is the case when user click a button, button produces Click event but it does
    /// not case who will handle it. Targeted events are useful to send some data to specific
    /// nodes.
    ///
    /// # Notes
    ///
    /// Even if event has `target` it still will be available to all other event handlers and
    /// listeners.
    pub target: Handle<UINode>,

    /// Source of event.
    pub(in crate) source: Handle<UINode>,
}

impl UIEvent {
    pub fn targeted(target: Handle<UINode>, kind: UIEventKind) -> Self {
        Self {
            kind,
            handled: false,
            source: Handle::NONE,
            target,
        }
    }

    pub fn new(kind: UIEventKind) -> Self {
        Self {
            kind,
            handled: false,
            source: Handle::NONE,
            target: Handle::NONE,
        }
    }

    pub fn source(&self) -> Handle<UINode> {
        self.source
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}

pub enum OsEvent {
    MouseInput {
        button: MouseButton,
        state: ButtonState,
    },
    CursorMoved {
        position: Vec2
    },
    KeyboardInput {
        button: KeyCode,
        state: ButtonState,
    },
    Character(char),
    MouseWheel(f32, f32),
}

#[derive(Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum KeyCode {
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    Key0,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,

    Escape,

    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,

    Snapshot,
    Scroll,
    Pause,

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left,
    Up,
    Right,
    Down,

    Backspace,
    Return,
    Space,

    Compose,

    Caret,

    Numlock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,

    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Calculator,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    LAlt,
    LBracket,
    LControl,
    LShift,
    LWin,
    Mail,
    MediaSelect,
    MediaStop,
    Minus,
    Multiply,
    Mute,
    MyComputer,
    NavigateForward,
    NavigateBackward,
    NextTrack,
    NoConvert,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    OEM102,
    Period,
    PlayPause,
    Power,
    PrevTrack,
    RAlt,
    RBracket,
    RControl,
    RShift,
    RWin,
    Semicolon,
    Slash,
    Sleep,
    Stop,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,
    Yen,
    Copy,
    Paste,
    Cut,
}