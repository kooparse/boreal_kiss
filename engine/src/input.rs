use std::collections::HashMap;

/// Game input system.
#[derive(Default)]
pub struct Input {
    keys: HashMap<Key, KeyState>,
    modifiers: Modifier,
}
impl Input {
    pub fn new() -> Self {
        Self {
            keys: HashMap::<Key, KeyState>::new(),
            ..Default::default()
        }
    }

    /// Add or remove a specified key if pressed or not.
    pub fn update_key(&mut self, keycode: Key, is_pressed: bool) {
        if is_pressed {
            self.add_key(keycode);
        } else {
            self.remove_key(keycode);
        }
    }

    /// Add keycode if not already there.
    pub fn add_key(&mut self, keycode: Key) {
        self.keys
            .entry(keycode)
            .or_insert_with(|| KeyState { once: false });
    }

    /// Return true is specified key is pressed.
    pub fn is_pressed(&self, keycode: Key) -> bool {
        self.keys.contains_key(&keycode)
    }

    /// Remove specified keycode.
    pub fn remove_key(&mut self, keycode: Key) {
        if self.keys.contains_key(&keycode) {
            self.keys.remove(&keycode);
        }
    }

    /// Set modifier.
    pub fn set_modifier(&mut self, modifier: Modifier) {
        self.modifiers = modifier;
    }

    /// Will call only once the closure when the given closure is pressed.
    pub fn once(&mut self, keycode: Key, mut callback: impl FnMut()) {
        if let Some(pressed) = self.keys.get_mut(&keycode) {
            if !pressed.once {
                pressed.once = true;
                callback();
            }
        }
    }

    pub fn clear(&mut self) {
        self.keys.clear();
        self.keys.shrink_to_fit();
    }
}

/// Key state, used for debounce.
pub struct KeyState {
    once: bool,
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

/// Mouse codes.
pub enum Mouse {
    Right,
    Left,
    CursorX,
    CursorY,
}
