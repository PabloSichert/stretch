use crate::geometry::{Point, Size};

#[derive(Debug)]
pub struct LayoutNode {
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<LayoutNode>,
}

pub type Node = LayoutNode;
