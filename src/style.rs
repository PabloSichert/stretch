use crate::geometry::{Rect, Size};
use crate::number::Number;
use crate::array::Array;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignItems {
    fn default() -> AlignItems {
        AlignItems::Stretch
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignSelf {
    Auto,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

impl Default for AlignSelf {
    fn default() -> AlignSelf {
        AlignSelf::Auto
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AlignContent {
    FlexStart,
    FlexEnd,
    Center,
    Stretch,
    SpaceBetween,
    SpaceAround,
}

impl Default for AlignContent {
    fn default() -> AlignContent {
        AlignContent::Stretch
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Direction {
    Inherit,
    LTR,
    RTL,
}

impl Default for Direction {
    fn default() -> Direction {
        Direction::Inherit
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Display {
    Flex,
    None,
}

impl Default for Display {
    fn default() -> Display {
        Display::Flex
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl Default for FlexDirection {
    fn default() -> FlexDirection {
        FlexDirection::Row
    }
}

impl FlexDirection {
    pub(crate) fn is_row(self) -> bool {
        self == FlexDirection::Row || self == FlexDirection::RowReverse
    }

    pub(crate) fn is_column(self) -> bool {
        self == FlexDirection::Column || self == FlexDirection::ColumnReverse
    }

    pub(crate) fn is_reverse(self) -> bool {
        self == FlexDirection::RowReverse || self == FlexDirection::ColumnReverse
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

impl Default for JustifyContent {
    fn default() -> JustifyContent {
        JustifyContent::FlexStart
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Overflow {
    Visible,
    Hidden,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Overflow {
        Overflow::Visible
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PositionType {
    Relative,
    Absolute,
}

impl Default for PositionType {
    fn default() -> PositionType {
        PositionType::Relative
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlexWrap {
    NoWrap,
    Wrap,
    WrapReverse,
}

impl Default for FlexWrap {
    fn default() -> FlexWrap {
        FlexWrap::NoWrap
    }
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Dimension {
    Undefined,
    Auto,
    Points(f32),
    Percent(f32),
}

impl Default for Dimension {
    fn default() -> Dimension {
        Dimension::Undefined
    }
}

impl Dimension {
    pub(crate) fn resolve(self, parent_width: Number) -> Number {
        match self {
            Dimension::Points(points) => Number::Defined(points),
            Dimension::Percent(percent) => parent_width * percent,
            _ => Number::Undefined,
        }
    }
}

impl Default for Rect<Dimension> {
    fn default() -> Rect<Dimension> {
        Rect { start: Default::default(), end: Default::default(), top: Default::default(), bottom: Default::default() }
    }
}

impl Default for Size<Dimension> {
    fn default() -> Size<Dimension> {
        Size { width: Dimension::Auto, height: Dimension::Auto }
    }
}

#[repr(C)]
pub struct StyleNode {
    pub position_type: PositionType,
    pub direction: Direction,
    pub flex_direction: FlexDirection,

    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,

    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,

    pub justify_content: JustifyContent,

    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,

    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,

    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,

    pub aspect_ratio: Number,

    pub children: Array<StyleNode>,
}

impl StyleNode {
    pub(crate) fn to_node(&self) -> Node {
        unsafe {
            let children = Vec::from_raw_parts(
                self.children.pointer,
                self.children.length,
                self.children.capacity
            ).iter().map(|child| child.to_node()).collect();

            Node {
                position_type: self.position_type,
                direction: self.direction,
                flex_direction: self.flex_direction,

                flex_wrap: self.flex_wrap,
                overflow: self.overflow,

                align_items: self.align_items,
                align_self: self.align_self,
                align_content: self.align_content,

                justify_content: self.justify_content,

                position: self.position,
                margin: self.margin,
                padding: self.padding,
                border: self.border,

                flex_grow: self.flex_grow,
                flex_shrink: self.flex_shrink,
                flex_basis: self.flex_basis,

                size: self.size,
                min_size: self.min_size,
                max_size: self.max_size,

                aspect_ratio: self.aspect_ratio,

                children: children,
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub position_type: PositionType,
    pub direction: Direction,
    pub flex_direction: FlexDirection,

    pub flex_wrap: FlexWrap,
    pub overflow: Overflow,

    pub align_items: AlignItems,
    pub align_self: AlignSelf,
    pub align_content: AlignContent,

    pub justify_content: JustifyContent,

    pub position: Rect<Dimension>,
    pub margin: Rect<Dimension>,
    pub padding: Rect<Dimension>,
    pub border: Rect<Dimension>,

    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: Dimension,

    pub size: Size<Dimension>,
    pub min_size: Size<Dimension>,
    pub max_size: Size<Dimension>,

    pub aspect_ratio: Number,

    pub children: Vec<Node>,
}

impl Default for Node {
    fn default() -> Node {
        Node {
            position_type: Default::default(),
            direction: Default::default(),
            flex_direction: Default::default(),

            flex_wrap: Default::default(),
            overflow: Default::default(),

            align_items: Default::default(),
            align_self: Default::default(),
            align_content: Default::default(),

            justify_content: Default::default(),

            position: Default::default(),
            margin: Default::default(),
            padding: Default::default(),
            border: Default::default(),

            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: Dimension::Auto,

            size: Default::default(),
            min_size: Default::default(),
            max_size: Default::default(),

            aspect_ratio: Default::default(),

            children: vec![],
        }
    }
}

impl Node {
    pub(crate) fn min_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_size.height,
        }
    }

    pub(crate) fn max_main_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_size.width,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_size.height,
        }
    }

    pub(crate) fn main_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.start,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.top,
        }
    }

    pub(crate) fn main_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.end,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.bottom,
        }
    }

    pub(crate) fn cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.size.width,
        }
    }

    pub(crate) fn min_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.min_size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.min_size.width,
        }
    }

    pub(crate) fn max_cross_size(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.max_size.height,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.max_size.width,
        }
    }

    pub(crate) fn cross_margin_start(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.top,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.start,
        }
    }

    pub(crate) fn cross_margin_end(&self, direction: FlexDirection) -> Dimension {
        match direction {
            FlexDirection::Row | FlexDirection::RowReverse => self.margin.bottom,
            FlexDirection::Column | FlexDirection::ColumnReverse => self.margin.end,
        }
    }

    pub(crate) fn align_self(&self, parent: &Node) -> AlignSelf {
        if self.align_self == AlignSelf::Auto {
            match parent.align_items {
                AlignItems::FlexStart => AlignSelf::FlexStart,
                AlignItems::FlexEnd => AlignSelf::FlexEnd,
                AlignItems::Center => AlignSelf::Center,
                AlignItems::Baseline => AlignSelf::Baseline,
                AlignItems::Stretch => AlignSelf::Stretch,
            }
        } else {
            self.align_self
        }
    }
}
