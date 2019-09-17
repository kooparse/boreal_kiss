use std::collections::HashMap;

/// Game input system.
#[derive(Default)]
pub struct Input {
    keyboard: HashMap<Key, KeyState>,
    mouse: HashMap<MouseButton, KeyState>,
    pub cursor: Cursor,
    modifiers: Modifier,
}
impl Input {
    pub fn new() -> Self {
        Self {
            keyboard: HashMap::<Key, KeyState>::new(),
            ..Default::default()
        }
    }

    /// Add or remove a specified key if pressed or not.
    pub fn update_key(&mut self, keycode: Key, is_pressed: bool) {
        if is_pressed {
            self.register_key(keycode);
        } else {
            self.remove_key(keycode);
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
        self.keyboard
            .entry(keycode)
            .or_insert_with(|| KeyState { once: false });
    }

    pub fn update_cursor_position(&mut self, position: Cursor) {
        self.cursor = position;
    }

    pub fn register_click(&mut self, button: MouseButton) {
        self.mouse
            .entry(button)
            .or_insert_with(|| KeyState { once: false });
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

    /// Return true is specified key is clicked.
    pub fn is_clicked(&self, button: MouseButton) -> bool {
        self.mouse.contains_key(&button)
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

    /// Will call only once the closure when the given closure is pressed.
    pub fn pressed_once(&mut self, keycode: Key, mut callback: impl FnMut()) {
        if let Some(pressed) = self.keyboard.get_mut(&keycode) {
            if !pressed.once {
                pressed.once = true;
                callback();
            }
        }
    }

    /// TODO: Should debounce/throttle here.
    pub fn on_cursor_moved(&self, mut callback: impl FnMut(&Cursor)) {
        if self.cursor.has_moved {
            callback(&self.cursor);
        }
    }

    /// Will call only once the closure when the given closure is pressed.
    pub fn clicked(
        &mut self,
        button: MouseButton,
        mut callback: impl FnMut(&Cursor),
    ) {
        if let Some(clicked) = self.mouse.get_mut(&button) {
            if !clicked.once {
                clicked.once = true;
                callback(&self.cursor);
            }
        }
    }

    pub fn clear(&mut self) {
        self.keyboard.clear();
        self.keyboard.shrink_to_fit();
    }
}

/// Key state, used for debounce.
pub struct KeyState {
    once: bool,
}

/// Mouse state.
#[derive(Default, Debug)]
pub struct Cursor {
    pub position: (f64, f64),
    // Used for 3D camera...
    pub delta: (f64, f64),
    pub has_moved: bool,
}

/// List of all keys available.
#[derive(Hash, Eq, PartialEq)]
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
