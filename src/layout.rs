use std::ffi::c_void;
use crate::geometry::{Point, Size};
use crate::array::Array;

#[repr(C)]
#[derive(Debug)]
pub struct LayoutNode {
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Array<c_void>,
}

#[derive(Debug)]
pub struct Node {
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Node>,
}

impl Node {
    pub(crate) unsafe fn to_layout_node(node: *const Node) -> Box<LayoutNode> {
        let children = Array {
            pointer: (*node).children.as_ptr() as *const c_void,
            length: (*node).children.len(),
            capacity: (*node).children.capacity(),
        };

        let layout = LayoutNode {
            size: (*node).size,
            location: (*node).location,
            children: children,
        };

        Box::new(layout)
    }
}
