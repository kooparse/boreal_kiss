use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Game input system.
#[derive(Default)]
pub struct Input {
    keyboard: HashMap<Key, KeyState>,
    mouse: HashMap<MouseButton, KeyState>,

    // Used for text input.
    pub last_key_pressed: Option<Key>,
    pub cursor: Cursor,
    pub modifiers: Modifier,
}
impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: HashMap::<Key, KeyState>::new(),
            ..Default::default()
        }
    }

    pub fn is_nothing_pressed(&self) -> bool {
        self.keyboard.is_empty()
    }

    /// Add or remove a specified key if pressed or not.
    pub fn update_key(&mut self, keycode: Key, is_pressed: bool) {
        if is_pressed {
            self.register_key(keycode);
            self.last_key_pressed = None;
        } else {
            self.remove_key(keycode);
            self.last_key_pressed = Some(keycode);
        }
    }

    /// Add or remove a specified key if clicked or not.
    pub fn update_mouse(&mut self, button: MouseButton, is_clicked: bool) {
        if is_clicked {
            self.register_click(button);
        } else {
            self.remove_click(button);
        }
    }

    /// Add keycode if not already there.
    pub fn register_key(&mut self, keycode: Key) {
        self.keyboard.entry(keycode).or_insert_with(|| KeyState {
            once: false,
            delay: Instant::now(),
        });
    }

    pub fn update_cursor_position(&mut self, position: Cursor) {
        self.cursor = position;
    }

    pub fn register_click(&mut self, button: MouseButton) {
        self.mouse.entry(button).or_insert_with(|| KeyState {
            once: false,
            delay: Instant::now(),
        });
    }

    pub fn remove_click(&mut self, button: MouseButton) {
        if self.mouse.contains_key(&button) {
            self.mouse.remove(&button);
        }
    }

    /// Return true is specified key is pressed.
    pub fn is_pressed(&self, keycode: Key) -> bool {
        self.keyboard.contains_key(&keycode)
    }

    pub fn is_pressed_delay(&mut self, delay: Duration, keycode: &Key) -> bool {
        if let Some(key) = self.keyboard.get_mut(&keycode) {
            if key.delay.elapsed() >= delay {
                key.delay = Instant::now();
                return true;
            }
            return false;
        } else {
            return false;
        }
    }

    pub fn pressed_str(&mut self) -> Option<String> {
        if let Some(last_key) = &self.last_key_pressed {
            let s = last_key.to_str();
            self.last_key_pressed = None;
            s
        } else {
            None
        }
    }

    /// Return true is specified key is clicked.
    pub fn is_clicked(&mut self, button: MouseButton) -> bool {
        let is_clicked = self.mouse.contains_key(&button);

        if is_clicked {
            self.cursor.is_dragged = true;
        } else {
            self.cursor.is_dragged = false;
        }

        is_clicked
    }

    /// Remove specified keycode.
    pub fn remove_key(&mut self, keycode: Key) {
        if self.keyboard.contains_key(&keycode) {
            self.keyboard.remove(&keycode);
        }
    }

    /// Set modifier.
    pub fn set_modifier(&mut self, modifier: Modifier) {
        self.modifiers = modifier;
    }

    pub fn is_pressed_once(&mut self, keycode: Key) -> bool {
        if let Some(pressed) = self.keyboard.get_mut(&keycode) {
            if !pressed.once {
                pressed.once = true;
                return true;
            }
        }

        false
    }

    /// TODO: Should debounce/throttle here.
    #[allow(unused)]
    pub fn is_cursor_moved(&self) -> bool {
        self.cursor.has_moved
    }

    /// TODO: Should debounce/throttle here.
    #[allow(unused)]
    pub fn is_dragged(&self) -> bool {
        self.cursor.is_dragged
    }

    /// Will call only once the closure when the given closure is pressed.
    pub fn is_clicked_once(&mut self, button: MouseButton) -> bool {
        if let Some(clicked) = self.mouse.get_mut(&button) {
            if !clicked.once {
                clicked.once = true;
                return true;
            }
        }

        false
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.keyboard.clear();
        self.keyboard.shrink_to_fit();
    }
}

/// Key state, used for debounce.
pub struct KeyState {
    once: bool,
    delay: Instant,
}

/// Mouse state.
#[derive(Default, Debug)]
pub struct Cursor {
    pub position: (f64, f64),
    // Used for 3D camera...
    pub delta: (f64, f64),
    pub has_moved: bool,
    pub is_dragged: bool,
}

/// List of all keys available.
#[derive(Hash, Eq, PartialEq, Copy, Clone)]
pub enum Key {
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
    Esc,
    Space,
    Tab,
    Bspc,
    Left,
    Down,
    Up,
    Right,
    Enter,
}

impl Key {
    fn to_str(&self) -> Option<String> {
        let value = match self {
            Key::A => Some("a"),
            Key::B => Some("b"),
            Key::C => Some("c"),
            Key::D => Some("d"),
            Key::E => Some("e"),
            Key::F => Some("f"),
            Key::G => Some("g"),
            Key::H => Some("h"),
            Key::I => Some("i"),
            Key::J => Some("j"),
            Key::K => Some("k"),
            Key::L => Some("l"),
            Key::M => Some("m"),
            Key::N => Some("n"),
            Key::O => Some("o"),
            Key::P => Some("p"),
            Key::Q => Some("q"),
            Key::R => Some("r"),
            Key::S => Some("s"),
            Key::T => Some("t"),
            Key::U => Some("u"),
            Key::V => Some("v"),
            Key::W => Some("w"),
            Key::X => Some("x"),
            Key::Y => Some("y"),
            Key::Z => Some("z"),
            Key::Space => Some(" "),
            Key::Key1 => Some("1"),
            Key::Key2 => Some("2"),
            Key::Key3 => Some("3"),
            Key::Key4 => Some("4"),
            Key::Key5 => Some("5"),
            Key::Key6 => Some("6"),
            Key::Key7 => Some("7"),
            Key::Key8 => Some("8"),
            Key::Key9 => Some("9"),
            Key::Key0 => Some("0"),
            _ => None,
        };

        value.map(|v| v.to_owned())
    }
}

#[derive(Default)]
pub struct Modifier {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub os: bool,
}

#[derive(Hash, Eq, PartialEq)]
pub enum MouseButton {
    Right,
    Left,
    Middle,
}
