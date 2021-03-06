use crate::{
    core::{
        color::Color,
        math::{
            vec2::Vec2,
            Rect,
        },
        pool::Handle,
    },
    VerticalAlignment,
    HorizontalAlignment,
    Thickness,
    Visibility,
    UserInterface,
    UINode,
    event::UIEvent,
    style::Style,
    Control,
    ControlTemplate,
};
use std::{
    cell::{
        RefCell,
        Cell,
    },
    collections::{
        VecDeque,
        HashMap,
    },
    any::Any,
    rc::Rc,
};

pub struct Widget {
    pub(in crate) name: String,
    /// Desired position relative to parent node
    pub(in crate) desired_local_position: Cell<Vec2>,
    /// Explicit width for node or automatic if NaN (means value is undefined). Default is NaN
    pub(in crate) width: Cell<f32>,
    /// Explicit height for node or automatic if NaN (means value is undefined). Default is NaN
    pub(in crate) height: Cell<f32>,
    /// Screen position of the node
    pub(in crate) screen_position: Vec2,
    /// Desired size of the node after Measure pass.
    pub(in crate) desired_size: Cell<Vec2>,
    /// Actual node local position after Arrange pass.
    pub(in crate) actual_local_position: Cell<Vec2>,
    /// Actual size of the node after Arrange pass.
    pub(in crate) actual_size: Cell<Vec2>,
    /// Minimum width and height
    pub(in crate) min_size: Vec2,
    /// Maximum width and height
    pub(in crate) max_size: Vec2,
    background: Color,
    foreground: Color,
    /// Index of row to which this node belongs
    row: usize,
    /// Index of column to which this node belongs
    column: usize,
    /// Vertical alignment
    pub(in crate) vertical_alignment: VerticalAlignment,
    /// Horizontal alignment
    pub(in crate) horizontal_alignment: HorizontalAlignment,
    /// Margin (four sides)
    pub(in crate) margin: Thickness,
    /// Current visibility state
    pub(in crate) visibility: Visibility,
    pub(in crate) global_visibility: bool,
    pub(in crate) children: Vec<Handle<UINode>>,
    pub(in crate) parent: Handle<UINode>,
    /// Indices of commands in command buffer emitted by the node.
    pub(in crate) command_indices: RefCell<Vec<usize>>,
    pub(in crate) is_mouse_over: bool,
    pub(in crate) measure_valid: Cell<bool>,
    pub(in crate) arrange_valid: Cell<bool>,
    pub(in crate) events: RefCell<VecDeque<UIEvent>>,
    pub(in crate) is_hit_test_visible: bool,
    pub(in crate) style: Option<Rc<Style>>,
}

impl Default for Widget {
    fn default() -> Self {
        WidgetBuilder::new().build()
    }
}

impl Control for Widget {
    fn widget(&self) -> &Widget {
        self
    }

    fn widget_mut(&mut self) -> &mut Widget {
        self
    }

    fn raw_copy(&self) -> Box<dyn Control> {
        Box::new(Self {
            name: self.name.clone(),
            desired_local_position: self.desired_local_position.clone(),
            width: self.width.clone(),
            height: self.height.clone(),
            screen_position: self.screen_position,
            desired_size: self.desired_size.clone(),
            actual_local_position: self.actual_local_position.clone(),
            actual_size: self.actual_size.clone(),
            min_size: self.min_size,
            max_size: self.max_size,
            background: self.background,
            foreground: self.foreground,
            row: self.row,
            column: self.column,
            vertical_alignment: self.vertical_alignment,
            horizontal_alignment: self.horizontal_alignment,
            margin: self.margin,
            visibility: self.visibility,
            global_visibility: self.global_visibility,
            children: self.children.clone(),
            parent: self.parent,
            command_indices: Default::default(),
            is_mouse_over: self.is_mouse_over,
            measure_valid: Cell::new(false),
            arrange_valid: Cell::new(false),
            events: Default::default(),
            is_hit_test_visible: self.is_hit_test_visible,
            style: self.style.clone(),
        })
    }

    fn resolve(&mut self, _: &ControlTemplate, _: &HashMap<Handle<UINode>, Handle<UINode>>) {}

    fn set_property(&mut self, name: &str, value: &dyn Any) {
        match name {
            Self::HORIZONTAL_ALIGNMENT => if let Some(value) = value.downcast_ref() {
                self.horizontal_alignment = *value
            },
            Self::VERTICAL_ALIGNMENT => if let Some(value) = value.downcast_ref() {
                self.vertical_alignment = *value
            },
            Self::WIDTH =>
                if let Some(value) = value.downcast_ref() {
                    self.width.set(*value)
                } else if let Some(value) = value.downcast_ref::<f64>() {
                    self.width.set(*value as f32)
                },
            Self::HEIGHT =>
                if let Some(value) = value.downcast_ref() {
                    self.height.set(*value)
                } else if let Some(value) = value.downcast_ref::<f64>() {
                    self.height.set(*value as f32)
                },
            Self::MARGIN => if let Some(value) = value.downcast_ref() {
                self.margin = *value
            },
            Self::ROW => if let Some(value) = value.downcast_ref() {
                self.row = *value
            },
            Self::COLUMN => if let Some(value) = value.downcast_ref() {
                self.column = *value
            },
            Self::BACKGROUND => if let Some(value) = value.downcast_ref() {
                self.background = *value
            },
            Self::FOREGROUND => if let Some(value) = value.downcast_ref() {
                self.foreground = *value;
            }
            Self::VISIBILITY => if let Some(value) = value.downcast_ref() {
                self.visibility = *value
            },
            Self::MIN_SIZE => if let Some(value) = value.downcast_ref() {
                self.min_size = *value
            },
            Self::MAX_SIZE => if let Some(value) = value.downcast_ref() {
                self.max_size = *value
            },
            _ => ()
        }
    }

    fn get_property(&self, name: &str) -> Option<&'_ dyn Any> {
        match name {
            Self::HORIZONTAL_ALIGNMENT => Some(&self.horizontal_alignment),
            Self::VERTICAL_ALIGNMENT => Some(&self.vertical_alignment),
            Self::WIDTH => Some(&self.width),
            Self::HEIGHT => Some(&self.height),
            Self::MARGIN => Some(&self.margin),
            Self::ROW => Some(&self.row),
            Self::COLUMN => Some(&self.column),
            Self::VISIBILITY => Some(&self.visibility),
            Self::BACKGROUND => Some(&self.background),
            Self::FOREGROUND => Some(&self.foreground),
            Self::MIN_SIZE => Some(&self.min_size),
            Self::MAX_SIZE => Some(&self.max_size),
            _ => None,
        }
    }
}

impl Widget {
    pub const WIDTH: &'static str = "Width";
    pub const HEIGHT: &'static str = "Height";
    pub const VERTICAL_ALIGNMENT: &'static str = "VerticalAlignment";
    pub const HORIZONTAL_ALIGNMENT: &'static str = "HorizontalAlignment";
    pub const MARGIN: &'static str = "Margin";
    pub const ROW: &'static str = "Row";
    pub const COLUMN: &'static str = "Column";
    pub const BACKGROUND: &'static str = "Background";
    pub const FOREGROUND: &'static str = "Foreground";
    pub const VISIBILITY: &'static str = "Visibility";
    pub const MIN_SIZE: &'static str = "MinSize";
    pub const MAX_SIZE: &'static str = "MaxSize";

    #[inline]
    pub fn set_name<P: AsRef<str>>(&mut self, name: P) -> &mut Self {
        self.name = name.as_ref().to_owned();
        self
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    #[inline]
    pub fn actual_size(&self) -> Vec2 {
        self.actual_size.get()
    }

    #[inline]
    pub fn set_min_size(&mut self, value: Vec2) -> &mut Self {
        self.min_size = value;
        self
    }

    #[inline]
    pub fn min_size(&self) -> Vec2 {
        self.min_size
    }

    #[inline]
    pub fn set_max_size(&mut self, value: Vec2) -> &mut Self {
        self.max_size = value;
        self
    }

    #[inline]
    pub fn max_size(&self) -> Vec2 {
        self.max_size
    }

    #[inline]
    pub fn set_background(&mut self, color: Color) -> &mut Self {
        self.background = color;
        self
    }

    #[inline]
    pub fn background(&self) -> Color {
        self.background
    }

    #[inline]
    pub fn set_foreground(&mut self, color: Color) -> &mut Self {
        self.foreground = color;
        self
    }

    #[inline]
    pub fn foreground(&self) -> Color {
        self.foreground
    }

    #[inline]
    pub fn set_width(&mut self, width: f32) -> &mut Self {
        self.width.set(width);
        self
    }

    #[inline]
    pub fn set_height(&mut self, height: f32) -> &mut Self {
        self.height.set(height);
        self
    }

    #[inline]
    pub fn set_desired_local_position(&mut self, pos: Vec2) -> &mut Self {
        self.desired_local_position.set(pos);
        self
    }

    #[inline]
    pub fn desired_local_position(&self) -> Vec2 {
        self.desired_local_position.get()
    }

    #[inline]
    pub fn set_vertical_alignment(&mut self, valign: VerticalAlignment) -> &mut Self {
        self.vertical_alignment = valign;
        self
    }

    #[inline]
    pub fn vertical_alignment(&self) -> VerticalAlignment {
        self.vertical_alignment
    }

    #[inline]
    pub fn set_horizontal_alignment(&mut self, halign: HorizontalAlignment) -> &mut Self {
        self.horizontal_alignment = halign;
        self
    }

    #[inline]
    pub fn horizontal_alignment(&self) -> HorizontalAlignment {
        self.horizontal_alignment
    }

    #[inline]
    pub fn set_column(&mut self, column: usize) -> &mut Self {
        self.column = column;
        self
    }

    #[inline]
    pub fn set_margin(&mut self, margin: Thickness) -> &mut Self {
        self.margin = margin;
        self
    }

    #[inline]
    pub fn margin(&self) -> Thickness {
        self.margin
    }

    #[inline]
    pub fn children(&self) -> &[Handle<UINode>] {
        &self.children
    }

    #[inline]
    pub fn parent(&self) -> Handle<UINode> {
        self.parent
    }

    #[inline]
    pub fn column(&self) -> usize {
        self.column
    }

    #[inline]
    pub fn set_row(&mut self, row: usize) -> &mut Self {
        self.row = row;
        self
    }

    #[inline]
    pub fn row(&self) -> usize {
        self.row
    }

    #[inline]
    pub fn get_screen_bounds(&self) -> Rect<f32> {
        Rect::new(
            self.screen_position.x,
            self.screen_position.y,
            self.actual_size.get().x,
            self.actual_size.get().y,
        )
    }

    #[inline]
    pub fn set_visibility(&mut self, visibility: Visibility) -> &mut Self {
        self.visibility = visibility;
        self
    }

    #[inline]
    pub fn get_visibility(&self) -> Visibility {
        self.visibility
    }

    #[inline]
    pub fn set_style(&mut self, style: Rc<Style>) -> &mut Self {
        self.style = Some(style);
        self
    }

    #[inline]
    pub fn style(&self) -> Option<Rc<Style>> {
        self.style.clone()
    }

    pub fn has_descendant(&self, node_handle: Handle<UINode>, ui: &UserInterface) -> bool {
        for child_handle in self.children.iter() {
            if *child_handle == node_handle {
                return true;
            }

            let result = ui.nodes.borrow(*child_handle).widget().has_descendant(node_handle, ui);
            if result {
                return true;
            }
        }
        false
    }

    /// Searches a node up on tree starting from given root that matches a criteria
    /// defined by a given func.
    pub fn find_by_criteria_up<Func: Fn(&UINode) -> bool>(&self, ui: &UserInterface, func: Func) -> Handle<UINode> {
        let mut parent_handle = self.parent;
        while parent_handle.is_some() {
            let parent_node = ui.nodes.borrow(parent_handle);
            if func(parent_node) {
                return parent_handle;
            }
            parent_handle = parent_node.widget().parent;
        }
        Handle::NONE
    }
}

pub struct WidgetBuilder {
    name: Option<String>,
    width: Option<f32>,
    height: Option<f32>,
    desired_position: Option<Vec2>,
    vertical_alignment: Option<VerticalAlignment>,
    horizontal_alignment: Option<HorizontalAlignment>,
    max_size: Option<Vec2>,
    min_size: Option<Vec2>,
    background: Option<Color>,
    foreground: Option<Color>,
    row: Option<usize>,
    column: Option<usize>,
    margin: Option<Thickness>,
    children: Vec<Handle<UINode>>,
    is_hit_test_visible: bool,
    visibility: Visibility,
    pub(in crate) style: Option<Rc<Style>>,
}

impl Default for WidgetBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl WidgetBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            width: None,
            height: None,
            vertical_alignment: None,
            horizontal_alignment: None,
            max_size: None,
            min_size: None,
            background: None,
            foreground: None,
            row: None,
            column: None,
            margin: None,
            desired_position: None,
            children: Vec::new(),
            is_hit_test_visible: true,
            visibility: Visibility::Visible,
            style: None,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width = Some(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = Some(height);
        self
    }

    pub fn with_vertical_alignment(mut self, valign: VerticalAlignment) -> Self {
        self.vertical_alignment = Some(valign);
        self
    }

    pub fn with_horizontal_alignment(mut self, halign: HorizontalAlignment) -> Self {
        self.horizontal_alignment = Some(halign);
        self
    }

    pub fn with_max_size(mut self, max_size: Vec2) -> Self {
        self.max_size = Some(max_size);
        self
    }

    pub fn with_min_size(mut self, min_size: Vec2) -> Self {
        self.min_size = Some(min_size);
        self
    }

    pub fn with_background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    pub fn with_foreground(mut self, color: Color) -> Self {
        self.foreground = Some(color);
        self
    }

    pub fn on_row(mut self, row: usize) -> Self {
        self.row = Some(row);
        self
    }

    pub fn on_column(mut self, column: usize) -> Self {
        self.column = Some(column);
        self
    }

    pub fn with_margin(mut self, margin: Thickness) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn with_desired_position(mut self, desired_position: Vec2) -> Self {
        self.desired_position = Some(desired_position);
        self
    }

    pub fn with_style(mut self, style: Rc<Style>) -> Self {
        self.style = Some(style);
        self
    }

    pub fn with_child(mut self, handle: Handle<UINode>) -> Self {
        if handle.is_some() {
            self.children.push(handle);
        }
        self
    }

    pub fn with_children(mut self, children: &[Handle<UINode>]) -> Self {
        for child in children {
            self.children.push(*child)
        }
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn with_hit_test_visibility(mut self, state: bool) -> Self {
        self.is_hit_test_visible = state;
        self
    }

    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    pub fn build(self) -> Widget {
        let mut widget = Widget {
            name: self.name.unwrap_or_default(),
            desired_local_position: Cell::new(self.desired_position.unwrap_or(Vec2::ZERO)),
            width: Cell::new(self.width.unwrap_or(std::f32::NAN)),
            height: Cell::new(self.height.unwrap_or(std::f32::NAN)),
            screen_position: Vec2::ZERO,
            desired_size: Cell::new(Vec2::ZERO),
            actual_local_position: Cell::new(Vec2::ZERO),
            actual_size: Cell::new(Vec2::ZERO),
            min_size: self.min_size.unwrap_or(Vec2::ZERO),
            max_size: self.max_size.unwrap_or_else(|| Vec2::new(std::f32::INFINITY, std::f32::INFINITY)),
            background: self.background.unwrap_or(Color::WHITE),
            foreground: self.foreground.unwrap_or(Color::WHITE),
            row: self.row.unwrap_or(0),
            column: self.column.unwrap_or(0),
            vertical_alignment: self.vertical_alignment.unwrap_or(VerticalAlignment::Stretch),
            horizontal_alignment: self.horizontal_alignment.unwrap_or(HorizontalAlignment::Stretch),
            margin: self.margin.unwrap_or_else(Thickness::zero),
            visibility: self.visibility,
            global_visibility: true,
            children: self.children,
            parent: Handle::NONE,
            command_indices: Default::default(),
            is_mouse_over: false,
            measure_valid: Cell::new(false),
            arrange_valid: Cell::new(false),
            events: RefCell::new(VecDeque::new()),
            is_hit_test_visible: self.is_hit_test_visible,
            style: None,
        };

        if let Some(style) = self.style {
            widget.apply_style(style);
        }

        widget
    }
}