use assets::Assets;
use colors::Colors;
use sprites::position::Position;
use sprites::sprite::{Draw, Positionate, Sprite, Update};
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};
use tetra::graphics::mesh::{BorderRadii, Mesh, ShapeStyle};
use tetra::graphics::text::Text;
use tetra::graphics::{Color, DrawParams, Rectangle};
use tetra::input::{Key, KeyModifier, MouseButton};
use tetra::math::Rect;
use tetra::{input, Context, TetraVec2};

enum ValueType {
    String { max_length: u32 },
    Unsigned { min: u32, max: u32 },
}

pub struct TextInput {
    id: String,
    text: Text,
    position: Position,
    width: f32,
    value_type: ValueType,
    rect: Option<Rect<f32, f32>>,
    is_focused: bool,
    is_disabled: bool,
    is_hovered: bool,
    is_danger: bool,
    blink: bool,
    last_blinked: Instant,
    dirty: bool,
    visible: bool,
}

impl TextInput {
    pub fn new(
        id: &str,
        value: &str,
        width: f32,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        Self {
            id: id.to_ascii_lowercase(),
            value_type: ValueType::String { max_length: 24 },
            text: Text::new(value, assets.borrow().default.clone()),
            position,
            width,
            rect: None,
            is_focused: false,
            is_disabled: false,
            is_hovered: false,
            is_danger: false,
            blink: false,
            last_blinked: Instant::now(),
            dirty: false,
            visible: true,
        }
    }

    pub fn int(
        id: &str,
        value: u32,
        clamps: (u32, u32),
        width: f32,
        assets: Rc<RefCell<Assets>>,
        position: Position,
    ) -> Self {
        let mut s = Self::new(id, format!("{}", value).as_str(), width, assets, position);
        s.value_type = ValueType::Unsigned {
            min: clamps.0,
            max: clamps.1,
        };
        s
    }

    fn border_color(&self) -> Color {
        if self.is_danger {
            Colors::DARK_RED
        } else if self.is_disabled {
            Colors::DARK_GRAY
        } else if self.is_focused {
            Colors::DARK_GREEN
        } else {
            Colors::DARK_BROWN
        }
    }

    fn bg_color(&self) -> Option<Color> {
        if self.is_danger && self.is_focused {
            Some(Colors::RED.with_alpha(0.8))
        } else if self.is_disabled {
            Some(Colors::DARK_GRAY.with_alpha(0.8))
        } else if self.is_focused {
            Some(Colors::DARK_GREEN.with_alpha(0.8))
        } else if self.is_hovered {
            Some(Colors::DARK_BROWN.with_alpha(0.2))
        } else {
            None
        }
    }

    fn text_color(&self) -> Color {
        if self.is_disabled {
            Colors::WHITE
        } else if self.is_focused {
            Colors::LIGHT_YELLOW
        } else {
            Colors::DARK_BROWN
        }
    }
}

impl Draw for TextInput {
    fn dirty(&self) -> bool {
        self.dirty
    }

    fn draw(&mut self, ctx: &mut Context) {
        let rect = self.rect.unwrap();
        if let Some(bg_color) = self.bg_color() {
            let bg = Mesh::rounded_rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(0.0, 0.0, rect.w, rect.h),
                BorderRadii::new(5.0),
            )
            .unwrap();
            bg.draw(
                ctx,
                DrawParams::new()
                    .position(TetraVec2::new(rect.x, rect.y))
                    .color(bg_color),
            );
        }

        let border = Mesh::rounded_rectangle(
            ctx,
            ShapeStyle::Stroke(2.0),
            Rectangle::new(0.0, 0.0, rect.w, rect.h),
            BorderRadii::new(5.0),
        )
        .unwrap();
        border.draw(
            ctx,
            DrawParams::new()
                .position(TetraVec2::new(rect.x, rect.y))
                .color(self.border_color()),
        );
        let content = self.text.content().to_string();
        let content_with_spaces = content.replace(" ", "_");
        self.text.set_content(content_with_spaces);
        let text_width = self
            .text
            .get_bounds(ctx)
            .map(|r| r.width + 3.0)
            .unwrap_or(-1.0f32);
        let text_pos = if !self.is_focused || self.is_disabled {
            TetraVec2::new(
                rect.x + rect.w / 2.0 - text_width / 2.0,
                rect.y + rect.h / 2.0 - 10.0,
            )
        } else {
            TetraVec2::new(rect.x + 7.0, rect.y + rect.h / 2.0 - 10.0)
        };
        self.text.set_content(content);
        self.text.draw(
            ctx,
            DrawParams::new()
                .position(text_pos)
                .color(self.text_color()),
        );
        if self.blink && self.is_focused {
            Mesh::rectangle(
                ctx,
                ShapeStyle::Fill,
                Rectangle::new(text_width + 7.0, rect.h / 2.0 - 10.0, 10.0, 20.0),
            )
            .unwrap()
            .draw(
                ctx,
                DrawParams::new()
                    .position(TetraVec2::new(rect.x, rect.y))
                    .color(self.text_color()),
            );
        }
        self.dirty = false;
    }

    fn visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}

impl Positionate for TextInput {
    fn position(&self) -> Position {
        self.position
    }

    fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    fn calc_size(&mut self, _ctx: &mut Context) -> TetraVec2 {
        TetraVec2::new(self.width, 42.0)
    }

    fn set_rect(&mut self, rect: Rect<f32, f32>) {
        self.rect = Some(rect);
    }
}

impl Update for TextInput {
    fn id(&self) -> Option<String> {
        Some(self.id.clone())
    }

    fn update(&mut self, ctx: &mut Context) -> Option<String> {
        let mouse = input::get_mouse_position(ctx);
        let collides = self.rect.unwrap().contains_point(mouse);
        if !self.is_hovered && collides {
            self.on_hovered();
        } else if self.is_hovered && !collides {
            self.off_hovered();
        }
        if self.is_focused {
            if input::is_mouse_button_pressed(ctx, MouseButton::Left) && !collides {
                self.off_pressed();
            }
            if Instant::now() - self.last_blinked > Duration::new(0, 500_000_000) {
                self.blink = !self.blink;
                self.last_blinked = Instant::now();
                self.dirty = true;
            }
            if input::is_key_pressed(ctx, Key::Backspace) && !self.text.content().is_empty() {
                self.text.pop();
                self.is_danger = false;
                self.dirty = true;
            }
            if let Some(text_input) = input::get_text_input(ctx) {
                let allow = match self.value_type {
                    ValueType::String { max_length } => {
                        (self.text.content().len() + text_input.len()) as u32 <= max_length
                    }
                    ValueType::Unsigned { .. } => matches!(text_input.parse::<u32>(), Ok(_)),
                };
                if allow {
                    self.text.push_str(text_input);
                    self.is_danger = false;
                    self.dirty = true;
                }
            }
            if let ValueType::String { max_length } = self.value_type {
                if (input::is_key_pressed(ctx, Key::V)
                    && input::is_key_modifier_down(ctx, KeyModifier::Ctrl))
                    || (input::is_key_pressed(ctx, Key::Insert)
                        && input::is_key_modifier_down(ctx, KeyModifier::Shift))
                {
                    let clipboard: String = input::get_clipboard_text(ctx)
                        .unwrap()
                        .chars()
                        .map(|c| if c == '\n' { ' ' } else { c })
                        .collect();
                    self.text.push_str(clipboard.as_str());
                    while self.text.content().len() as u32 > max_length {
                        self.text.pop();
                    }
                    self.is_danger = false;
                    self.dirty = true;
                }
            }
        } else if input::is_mouse_button_pressed(ctx, MouseButton::Left) && collides {
            self.on_pressed();
        }
        None
    }
}

impl Sprite for TextInput {
    fn on_pressed(&mut self) {
        self.is_focused = true;
        self.blink = true;
        self.last_blinked = Instant::now();
        self.dirty = true;
    }

    fn off_pressed(&mut self) {
        self.is_focused = false;
        self.blink = false;
        self.dirty = true;
        self.validate_value();
    }

    fn on_hovered(&mut self) {
        self.is_hovered = true;
        self.dirty = true;
    }

    fn off_hovered(&mut self) {
        self.is_hovered = false;
        self.dirty = true;
    }

    fn is_focusable(&self) -> bool {
        !self.is_disabled
    }

    fn set_value(&mut self, value: &str) {
        self.text.set_content(value);
        self.is_danger = false;
        self.dirty = true;
        self.validate_value();
    }

    fn get_value(&self) -> Option<String> {
        Some(self.text.content().to_string())
    }

    fn validate_value(&mut self) {
        if let ValueType::Unsigned { min, max } = self.value_type {
            let mut val = self.text.content().parse::<u32>().unwrap_or(min);
            if val < min {
                val = min;
            } else if val > max {
                val = max;
            }
            self.text.set_content(format!("{}", val));
        }
    }

    fn set_danger(&mut self, danger: bool) {
        self.is_danger = danger;
        self.dirty = true;
    }

    fn get_danger(&self) -> bool {
        self.is_danger
    }
}
