use crate::global::SCREEN_HEIGHT;
use crate::input::{Input, Key, MouseButton};
use crate::renderer::{
    draw_quad, draw_text, primitives, Font, Mesh, Rgb, Rgba, Text, Transform,
    Vector,
};
use std::fmt::Debug;

#[derive(Debug, Copy, Clone)]
pub struct Styles {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub width: f32,
    pub padding: f32,
    pub margin: f32,
    pub text_color: Rgb,
    pub bg_color: Rgba,
    pub font_size: f32,
    pub opacity: f32,

    // Used for children if any.
    pub align: Align,
}

impl Default for Styles {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            height: 0.,
            width: 0.,
            padding: 0.,
            margin: 0.,
            font_size: 16.,
            text_color: Rgb::new(1., 1., 1.),
            bg_color: Rgba::new(0., 0., 1., 1.),
            opacity: 1.,
            align: Align::Row,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Align {
    Row,
    Col,
}

/// Generic wrapper for widget trait.
/// Store the given widget on the heap.
pub struct Element {
    widget: Box<dyn Widget>,
}

impl Element {
    fn new(widget: impl Widget + 'static) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }
}

/// TODO: Remove the required font.
pub trait Widget {
    /// Used to react from window events.
    fn on_event(&mut self, input: &mut Input);

    /// Updates all widget positions.
    /// Parents could influence the position of their children.
    /// It should be called each new widget added into the GUI.
    fn compute_layout(&mut self, position: (f32, f32));
    /// Draw the widget.
    fn draw(&mut self, quad: &Mesh, font: &mut Font);

    /// Get the style struct of the widget.
    fn get_styles(&self) -> &Styles;
    /// Get the position.
    fn get_pos(&self) -> (f32, f32);
    /// Get the dimension.
    fn get_dim(&self) -> (f32, f32);
}

pub struct GUI {
    pub quad: Mesh,
    pub elements: Vec<Element>,
}

impl GUI {
    pub fn new() -> Self {
        let quad = primitives::create_quad(Transform::default());
        Self {
            quad,
            elements: vec![],
        }
    }

    pub fn add_elem(mut self, widget: impl Widget + 'static) -> Self {
        self.elements.push(Element::new(widget));
        self
    }

    pub fn on_event(&mut self, mut input: &mut Input) {
        for elem in self.elements.iter_mut() {
            elem.widget.on_event(&mut input);
        }
    }

    pub fn draw(&mut self, mut font: &mut Font) {
        for elem in self.elements.iter_mut() {
            elem.widget.draw(&self.quad, &mut font);
        }
    }
}

/// Like a "div" in html/css.
/// Used build pattern.
pub struct Container {
    pub styles: Styles,
    pub align: Align,
    pub content: Vec<Element>,
}

impl Container {
    pub fn row() -> Self {
        Self {
            styles: Styles::default(),
            align: Align::Row,
            content: vec![],
        }
    }

    pub fn col() -> Self {
        Self {
            styles: Styles::default(),
            align: Align::Col,
            content: vec![],
        }
    }

    pub fn height(mut self, height: f32) -> Self {
        self.styles.height = height;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.styles.width = width;
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.styles.margin = margin;
        self
    }

    pub fn push(mut self, widget: impl Widget + 'static) -> Self {
        let elem = Element::new(widget);
        self.content.push(elem);
        self.compute_layout((0., 0.));
        self
    }
}

impl Widget for Container {
    fn on_event(&mut self, mut input: &mut Input) {
        for elem in self.content.iter_mut() {
            elem.widget.on_event(&mut input);
        }
    }

    fn compute_layout(&mut self, position: (f32, f32)) {
        let mut position = (
            self.styles.x + self.styles.margin + position.0,
            self.styles.y + self.styles.margin + position.1,
        );

        if self.styles.height > 0. {
            position.1 += self.styles.height - self.get_dim().1;
            position.1 -= self.styles.margin;
        }

        for e in self.content.iter_mut() {
            let dim = e.widget.get_dim();
            e.widget.compute_layout(position);

            match self.align {
                Align::Row => {
                    position.0 += dim.0;
                }
                Align::Col => {
                    position.1 += dim.1;
                }
            }
        }
    }

    fn draw(&mut self, quad: &Mesh, mut font: &mut Font) {
        self.content.iter_mut().for_each(|elem| {
            elem.widget.draw(&quad, &mut font);
        })
    }

    fn get_pos(&self) -> (f32, f32) {
        (self.styles.x, self.styles.y)
    }

    fn get_styles(&self) -> &Styles {
        &self.styles
    }

    fn get_dim(&self) -> (f32, f32) {
        let margin = self.styles.margin;
        let (mut width, mut height) = (margin, margin);

        for elem in self.content.iter() {
            let dim = elem.widget.get_dim();

            match self.align {
                Align::Row => {
                    width += dim.0;
                    if dim.1 >= height {
                        height = dim.1 + margin;
                    }
                }
                Align::Col => {
                    height += dim.1;
                    if dim.0 >= width {
                        width = dim.0 + margin;
                    }
                }
            }
        }

        (width, height)
    }
}

pub struct TextInput {
    pub styles: Styles,
    pub value: Text,
    pub label: Option<Text>,
    // pub label: String,
    pub is_focus: bool,
    pub is_hover: bool,
    pub only_numbers: bool,

    pub on_change: Box<dyn FnMut(&Text)>,
}

impl TextInput {
    pub fn new() -> Self {
        let styles = Styles {
            width: 200.,
            height: 40.,
            ..Styles::default()
        };

        let value = Text::new("").font_size(26.).color(styles.text_color);

        Self {
            styles,
            value,
            is_focus: false,
            is_hover: false,
            label: None,
            only_numbers: false,
            on_change: Box::new(|_| {}),
        }
    }

    pub fn only_numbers(mut self, only_n: bool) -> Self {
        self.only_numbers = only_n;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.styles.padding = padding;
        self
    }

    pub fn value(mut self, value: Text) -> Self {
        self.value = value;
        self
    }

    pub fn label(mut self, label: Text) -> Self {
        self.label = Some(label);
        self
    }

    pub fn on_update(mut self, cb: impl FnMut(&Text) + 'static) -> Self {
        self.on_change = Box::new(cb);
        self
    }
}

impl Widget for TextInput {
    fn on_event(&mut self, input: &mut Input) {
        let (bottom_left, top_right) = {
            let bottom_left = (self.styles.x, self.styles.y);
            let top_right = (
                self.styles.x + self.styles.width,
                self.styles.y + self.styles.height,
            );

            (bottom_left, top_right)
        };

        let (cx, cy) = unsafe {
            (
                input.cursor.position.0 as f32,
                SCREEN_HEIGHT - input.cursor.position.1 as f32,
            )
        };

        self.is_hover = cx >= bottom_left.0
            && cx <= top_right.0
            && cy >= bottom_left.1
            && cy <= top_right.1;

        if input.is_clicked_once(MouseButton::Left) {
            self.is_focus = self.is_hover;
        }

        if !self.is_focus {
            return;
        }

        let mut has_value_changed = false;
        if let Some(value) = input.pressed_str() {
            if self.only_numbers {
                if value.parse::<f32>().is_ok() {
                    self.value.content.push_str(value.as_str());
                    has_value_changed = true;
                }
            } else {
                self.value.content.push_str(value.as_str());
                has_value_changed = true;
            }
        }

        if input.is_pressed_once(Key::Bspc) {
            self.value.content.pop();
            has_value_changed = true;
        }

        if has_value_changed {
            (self.on_change)(&self.value);
        }
    }

    fn compute_layout(&mut self, pos: (f32, f32)) {
        self.styles.x = pos.0;
        self.styles.y = pos.1;

        if let Some(label) = &mut self.label {
            label.position = Vector(pos.0, pos.1, 0.);
        }

        self.value.position =
            Vector(pos.0, pos.1 + self.styles.height * 0.5, 0.);
    }

    fn draw(&mut self, quad: &Mesh, font: &mut Font) {
        let Styles {
            x,
            y,
            width,
            height,
            padding,
            bg_color,
            ..
        } = self.styles;

        // 1. Draw the input box.
        //
        let mut t = Transform::default();
        t.position = Vector(x, y, 0.);
        t.scale = Vector(width, height, 1.);

        draw_quad(&quad, &t, bg_color);

        // 2. Draw the value of the input.
        //
        let txt_length = font.text_length(&self.value);
        let size_ratio = self.value.font_size / font.size;
        self.value.position = Vector(
            x + padding,
            (y + height * 0.5) + (-0.5 * txt_length.1 * size_ratio),
            0.,
        );
        draw_text(font, &self.value);

        // 3. If label, draw the label.
        //
        if let Some(label) = &mut self.label {
            let padding = 5.;
            label.position = Vector(x, y + height + padding, 0.);
            draw_text(font, &label);
        }
    }

    fn get_styles(&self) -> &Styles {
        &self.styles
    }

    fn get_pos(&self) -> (f32, f32) {
        (self.styles.x, self.styles.y)
    }

    fn get_dim(&self) -> (f32, f32) {
        let mut dim = (self.styles.width, self.styles.height);
        if let Some(_) = &self.label {
            dim.1 += self.styles.height * 0.7;
        }

        dim
    }
}

pub struct Button {
    // Graphic button stuff.
    pub styles: Styles,
    pub text: Text,

    // Button state.
    pub is_displayed: bool,
    pub is_hover: bool,

    pub action: Box<dyn FnMut(&mut Text)>,
}

impl Button {
    pub fn new(text: Text) -> Self {
        let styles = Styles {
            width: 80.,
            height: 30.,
            ..Styles::default()
        };

        Self {
            text,
            styles,
            is_displayed: true,
            is_hover: false,
            action: Box::new(|_| {}),
        }
    }

    pub fn callback(mut self, cb: impl FnMut(&mut Text) + 'static) -> Self {
        self.action = Box::new(cb);
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.styles.width = width;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.styles.height = height;
        self
    }

    pub fn margin(mut self, margin: f32) -> Self {
        self.styles.margin = margin;
        self
    }

    pub fn text_color(mut self, color: Rgb) -> Self {
        self.styles.text_color = color;
        self
    }

    pub fn bg_color(mut self, color: Rgba) -> Self {
        self.styles.bg_color = color;
        self
    }

    #[allow(unused)]
    pub fn font_size(mut self, size: f32) -> Self {
        self.styles.font_size = size;
        self
    }
}

impl Widget for Button {
    fn on_event(&mut self, input: &mut Input) {
        let (bottom_left, top_right) = {
            let bottom_left = (self.styles.x, self.styles.y);
            let top_right = (
                self.styles.x + self.styles.width,
                self.styles.y + self.styles.height,
            );

            (bottom_left, top_right)
        };

        let (cx, cy) = unsafe {
            (
                input.cursor.position.0 as f32,
                SCREEN_HEIGHT - input.cursor.position.1 as f32,
            )
        };

        self.is_hover = cx >= bottom_left.0
            && cx <= top_right.0
            && cy >= bottom_left.1
            && cy <= top_right.1;

        if self.is_hover && input.is_clicked_once(MouseButton::Left) {
            (self.action)(&mut self.text);
        }
    }

    fn get_styles(&self) -> &Styles {
        &self.styles
    }

    fn get_pos(&self) -> (f32, f32) {
        (self.styles.x, self.styles.y)
    }

    fn get_dim(&self) -> (f32, f32) {
        (
            self.styles.width + self.styles.margin,
            self.styles.height + self.styles.margin,
        )
    }

    fn compute_layout(&mut self, position: (f32, f32)) {
        self.styles.x = position.0;
        self.styles.y = position.1;
    }

    fn draw(&mut self, quad: &Mesh, font: &mut Font) {
        let Styles {
            x,
            y,
            width,
            height,
            bg_color,
            ..
        } = self.styles;

        let mut t = Transform::default();
        t.position = Vector(x, y, 0.);
        t.scale = Vector(width, height, 1.);
        draw_quad(&quad, &t, bg_color);

        let txt_length = font.text_length(&self.text);

        let size_ratio = self.text.font_size / font.size;
        self.text.position = Vector(
            (x + width * 0.5) + (-0.5 * txt_length.0 * size_ratio),
            (y + height * 0.5) + (-0.5 * txt_length.1 * size_ratio),
            0.,
        );

        draw_text(font, &self.text)
    }
}
