use crate::{
    core::{
        math::{
            vec2::Vec2,
            Rect,
        },
        pool::Handle,
        color::Color,
    },
    widget::{
        WidgetBuilder,
        Widget,
    },
    UserInterface,
    draw::{
        DrawingContext,
        CommandKind,
        CommandTexture,
    },
    formatted_text::{
        FormattedText,
        FormattedTextBuilder,
    },
    UINode,
    Control,
    event::{
        UIEvent,
        UIEventKind,
        MouseButton,
        KeyCode
    },
    ControlTemplate,
    UINodeContainer,
    Builder,
    ttf::Font,
};
use std::{
    collections::HashMap,
    cmp,
    sync::{Mutex, Arc},
    cell::RefCell
};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum HorizontalDirection {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum VerticalDirection {
    Down,
    Up,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Position {
    line: usize,
    offset: usize,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct SelectionRange {
    begin: Position,
    end: Position,
}

pub struct TextBox {
    widget: Widget,
    caret_line: usize,
    caret_offset: usize,
    caret_visible: bool,
    blink_timer: f32,
    blink_interval: f32,
    formatted_text: RefCell<FormattedText>,
    selection_range: Option<SelectionRange>,
    selecting: bool,
}

impl TextBox {
    pub fn new(widget: Widget) -> Self {
        Self {
            widget,
            caret_line: 0,
            caret_offset: 0,
            caret_visible: false,
            blink_timer: 0.0,
            blink_interval: 0.0,
            formatted_text: RefCell::new(FormattedTextBuilder::new()
                .with_font(crate::DEFAULT_FONT.clone())
                .build()),
            selection_range: None,
            selecting: false,
        }
    }

    pub fn reset_blink(&mut self) {
        self.caret_visible = true;
        self.blink_timer = 0.0;
    }

    pub fn move_caret_x(&mut self, mut offset: usize, direction: HorizontalDirection) {
        self.selection_range = None;

        self.reset_blink();

        let text = self.formatted_text.borrow();
        let lines = text.get_lines();

        if lines.is_empty() {
            self.caret_offset = 0;
            self.caret_line = 0;
            return;
        }

        while offset > 0 {
            match direction {
                HorizontalDirection::Left => {
                    if self.caret_offset > 0 {
                        self.caret_offset -= 1
                    } else if self.caret_line > 0 {
                        self.caret_line -= 1;
                        self.caret_offset = lines[self.caret_line].len();
                    } else {
                        self.caret_offset = 0;
                        break;
                    }
                }
                HorizontalDirection::Right => {
                    let line = lines.get(self.caret_line).unwrap();
                    if self.caret_offset < line.len() {
                        self.caret_offset += 1;
                    } else if self.caret_line < lines.len() - 1 {
                        self.caret_line += 1;
                        self.caret_offset = 0;
                    } else {
                        self.caret_offset = line.len();
                        break;
                    }
                }
            }
            offset -= 1;
        }
    }

    pub fn move_caret_y(&mut self, offset: usize, direction: VerticalDirection) {
        let text = self.formatted_text.borrow();
        let lines = text.get_lines();

        if lines.is_empty() {
            return;
        }

        let line_count = lines.len();

        match direction {
            VerticalDirection::Down => {
                if self.caret_line + offset >= line_count {
                    self.caret_line = line_count - 1;
                } else {
                    self.caret_line += offset;
                }
            }
            VerticalDirection::Up => {
                if self.caret_line > offset {
                    self.caret_line -= offset;
                } else {
                    self.caret_line = 0;
                }
            }
        }
    }

    pub fn get_absolute_position(&self) -> Option<usize> {
        if let Some(line) = self.formatted_text.borrow().get_lines().get(self.caret_line) {
            Some(line.begin + cmp::min(self.caret_offset, line.len()))
        } else {
            None
        }
    }

    /// Inserts given character at current caret position.
    pub fn insert_char(&mut self, c: char) {
        if !c.is_control() {
            let position = self.get_absolute_position().unwrap_or(0);
            self.formatted_text.borrow_mut().insert_char(c, position);
            self.formatted_text.borrow_mut().build();
            self.move_caret_x(1, HorizontalDirection::Right);
        }
    }

    pub fn get_text_len(&self) -> usize {
        self.formatted_text.borrow_mut().get_raw_text().len()
    }

    pub fn remove_char(&mut self, direction: HorizontalDirection) {
        if let Some(position) = self.get_absolute_position() {
            let text_len = self.get_text_len();
            if text_len != 0 {
                let position = match direction {
                    HorizontalDirection::Left => {
                        if position == 0 {
                            return;
                        }
                        position - 1
                    }
                    HorizontalDirection::Right => {
                        if position >= text_len {
                            return;
                        }
                        position
                    }
                };
                self.formatted_text.borrow_mut().remove_at(position);
                self.formatted_text.borrow_mut().build();

                if direction == HorizontalDirection::Left {
                    self.move_caret_x(1, direction);
                }
            }
        }
    }

    pub fn screen_pos_to_text_pos(&self, screen_pos: Vec2) -> Option<Position> {
        let mut caret_pos = self.widget.screen_position;
        if let Some(font) = self.formatted_text.borrow().get_font() {
            let font = font.lock().unwrap();
            for (line_index, line) in self.formatted_text.borrow().get_lines().iter().enumerate() {
                let line_bounds =
                    Rect::new(caret_pos.x + line.x_offset, caret_pos.y, line.width, font.get_ascender());
                if line_bounds.contains(screen_pos.x, screen_pos.y) {
                    let mut x = line_bounds.x;
                    // Check each character in line.
                    for (offset, index) in (line.begin..line.end).enumerate() {
                        let symbol = self.formatted_text.borrow().get_raw_text()[index];
                        let (width, height, advance) = if let Some(glyph) = font.get_glyph(symbol) {
                            (glyph.get_bitmap_width(), glyph.get_bitmap_height(), glyph.get_advance())
                        } else {
                            // Stub
                            let h = font.get_height();
                            (h, h, h)
                        };
                        let char_bounds = Rect::new(x, line_bounds.y, width, height);
                        if char_bounds.contains(screen_pos.x, screen_pos.y) {
                            return Some(Position { line: line_index, offset });
                        }
                        x += advance;
                    }
                }
                caret_pos.y += line_bounds.h;
            }
        }
        None
    }
}

impl Control for TextBox {
    fn widget(&self) -> &Widget {
        &self.widget
    }

    fn widget_mut(&mut self) -> &mut Widget {
        &mut self.widget
    }

    fn raw_copy(&self) -> Box<dyn Control> {
        Box::new(Self {
            widget: *self.widget.raw_copy().downcast::<Widget>().unwrap_or_else(|_| panic!()),
            caret_line: self.caret_line,
            caret_offset: self.caret_offset,
            caret_visible: self.caret_visible,
            blink_timer: self.blink_timer,
            blink_interval: self.blink_interval,
            formatted_text: RefCell::new(FormattedTextBuilder::new()
                .with_font(self.formatted_text.borrow().get_font().unwrap()).build()),
            selection_range: self.selection_range,
            selecting: self.selecting,
        })
    }

    fn resolve(&mut self, _: &ControlTemplate, _: &HashMap<Handle<UINode>, Handle<UINode>>) {}

    fn draw(&self, drawing_context: &mut DrawingContext) {
        self.widget.draw(drawing_context);

        let bounds = self.widget.get_screen_bounds();
        drawing_context.push_rect_filled(&bounds, None, Color::opaque(80, 80, 80));
        drawing_context.commit(CommandKind::Geometry, CommandTexture::None);

        self.formatted_text.borrow_mut().set_size(Vec2::new(bounds.w, bounds.h));
        self.formatted_text.borrow_mut().set_color(self.widget.background());
        self.formatted_text.borrow_mut().build();

        if let Some(ref selection_range) = self.selection_range {
            let text = self.formatted_text.borrow();
            let lines = text.get_lines();
            if selection_range.begin.line == selection_range.end.line {
                let line = lines[selection_range.begin.line];
                let begin = selection_range.begin.offset;
                let end = selection_range.end.offset;
                // Begin line
                let offset = text.get_range_width(line.begin..(line.begin + begin));
                let width = text.get_range_width((line.begin + begin)..(line.begin + end));
                let bounds = Rect::new(bounds.x + line.x_offset + offset,
                                       bounds.y + line.y_offset,
                                       width,
                                       line.height);
                drawing_context.push_rect_filled(&bounds, None, Color::opaque(65, 65, 90));
            } else {
                for (i, line) in text.get_lines().iter().enumerate() {
                    if i >= selection_range.begin.line && i <= selection_range.end.line {
                        let bounds = if i == selection_range.begin.line {
                            // Begin line
                            let offset = text.get_range_width(line.begin..(line.begin + selection_range.begin.offset));
                            let width = text.get_range_width((line.begin + selection_range.begin.offset)..line.end);
                            Rect::new(bounds.x + line.x_offset + offset,
                                      bounds.y + line.y_offset,
                                      width,
                                      line.height)
                        } else if i == selection_range.end.line {
                            // End line
                            let width = text.get_range_width(line.begin..(line.begin + selection_range.end.offset));
                            Rect::new(bounds.x + line.x_offset,
                                      bounds.y + line.y_offset,
                                      width,
                                      line.height)
                        } else {
                            // Everything between
                            Rect::new(bounds.x + line.x_offset,
                                      bounds.y + line.y_offset,
                                      line.width,
                                      line.height)
                        };
                        drawing_context.push_rect_filled(&bounds, None, Color::opaque(90, 90, 120));
                    }
                }
            }
        }
        drawing_context.commit(CommandKind::Geometry, CommandTexture::None);

        let screen_position = Vec2::new(bounds.x, bounds.y);
        drawing_context.draw_text(screen_position, &self.formatted_text.borrow());

        if self.caret_visible {
            let text = self.formatted_text.borrow();
            if let Some(font) = text.get_font() {
                let font = font.lock().unwrap();
                if let Some(line) = text.get_lines().get(self.caret_line) {
                    let text = text.get_raw_text();
                    let mut caret_pos = Vec2::new(
                        screen_position.x,
                        screen_position.y + self.caret_line as f32 * font.get_ascender(),
                    );
                    for (offset, char_index) in (line.begin..line.end).enumerate() {
                        if offset >= self.caret_offset {
                            break;
                        }
                        if let Some(glyph) = font.get_glyph(text[char_index]) {
                            caret_pos.x += glyph.get_advance();
                        } else {
                            caret_pos.x += font.get_height();
                        }
                    }

                    let caret_bounds = Rect::new(caret_pos.x, caret_pos.y, 2.0, font.get_height());
                    drawing_context.push_rect_filled(&caret_bounds, None, Color::WHITE);
                    drawing_context.commit(CommandKind::Geometry, CommandTexture::None);
                }
            }
        }
    }

    fn update(&mut self, dt: f32) {
        self.widget.update(dt);
        self.blink_timer += dt;
        if self.blink_timer >= self.blink_interval {
            self.blink_timer = 0.0;
            self.caret_visible = !self.caret_visible;
        }
    }

    fn handle_event(&mut self, self_handle: Handle<UINode>, ui: &mut UserInterface, evt: &mut UIEvent) {
        if evt.source == self_handle || self.widget().has_descendant(evt.source, ui) {
            match evt.kind {
                UIEventKind::Text { symbol } => {
                    self.insert_char(symbol);
                }
                UIEventKind::KeyDown { code } => {
                    match code {
                        KeyCode::Up => {
                            self.move_caret_y(1, VerticalDirection::Up);
                        }
                        KeyCode::Down => {
                            self.move_caret_y(1, VerticalDirection::Down);
                        }
                        KeyCode::Right => {
                            self.move_caret_x(1, HorizontalDirection::Right);
                        }
                        KeyCode::Left => {
                            self.move_caret_x(1, HorizontalDirection::Left);
                        }
                        KeyCode::Delete => {
                            self.remove_char(HorizontalDirection::Right);
                        }
                        KeyCode::Backspace => {
                            self.remove_char(HorizontalDirection::Left);
                        }
                        _ => ()
                    }
                }
                UIEventKind::MouseDown { pos, button } => {
                    if button == MouseButton::Left {
                        self.selection_range = None;
                        self.selecting = true;

                        if let Some(position) = self.screen_pos_to_text_pos(pos) {
                            self.caret_line = position.line;
                            self.caret_offset = position.offset;

                            self.selection_range = Some(SelectionRange {
                                begin: position,
                                end: position,
                            })
                        }

                        ui.capture_mouse(self_handle);
                    }
                }
                UIEventKind::MouseMove { pos } => {
                    if self.selecting {
                        if let Some(position) = self.screen_pos_to_text_pos(pos) {
                            if let Some(ref mut sel_range) = self.selection_range {
                                sel_range.end = position;
                            }
                        }
                    }
                }
                UIEventKind::MouseUp { .. } => {
                    self.selecting = false;

                    ui.release_mouse_capture();
                }
                _ => {}
            }
        }
    }
}

pub struct TextBoxBuilder {
    widget_builder: WidgetBuilder,
    font: Option<Arc<Mutex<Font>>>,
    text: String,
}

impl TextBoxBuilder {
    pub fn new(widget_builder: WidgetBuilder) -> Self {
        Self {
            widget_builder,
            font: None,
            text: "".to_owned(),
        }
    }

    pub fn with_font(mut self, font: Arc<Mutex<Font>>) -> Self {
        self.font = Some(font);
        self
    }

    pub fn with_text(mut self, text: String) -> Self {
        self.text = text;
        self
    }
}

impl Builder for TextBoxBuilder {
    fn build(self, ui: &mut dyn UINodeContainer) -> Handle<UINode> {
        let text_box = TextBox {
            widget: self.widget_builder.build(),
            caret_line: 0,
            caret_offset: 0,
            caret_visible: true,
            blink_timer: 0.0,
            blink_interval: 0.5,
            formatted_text: RefCell::new(FormattedTextBuilder::new()
                .with_text(self.text)
                .with_font(self.font.unwrap_or(crate::DEFAULT_FONT.clone()))
                .build()),
            selection_range: None,
            selecting: false,
        };

        ui.add_node(Box::new(text_box))
    }
}
