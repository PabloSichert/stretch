use std::mem;
use crate::geometry::{Point, Size};
use crate::array::Array;

#[repr(C)]
#[derive(Debug)]
pub struct LayoutNode {
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Array<LayoutNode>,
}

impl LayoutNode {
    pub(crate) fn to_node(&self) -> Node {
        unsafe {
            let children = Vec::from_raw_parts(
                self.children.pointer,
                self.children.length,
                self.children.capacity
            ).iter().map(|child| child.to_node()).collect();

            Node {
                size: self.size,
                location: self.location,
                children: children,
            }
        }
    }
}

#[derive(Debug)]
pub struct Node {
    pub size: Size<f32>,
    pub location: Point<f32>,
    pub children: Vec<Node>,
}

impl Node {
    pub(crate) fn to_layout_node(&self) -> LayoutNode {
        let mut children_vec: Vec<LayoutNode> = self.children.iter().map(|child| child.to_layout_node()).collect();

        children_vec.shrink_to_fit();

        let children = Array {
            pointer: children_vec.as_mut_ptr(),
            length: children_vec.len(),
            capacity: children_vec.capacity()
        };

        mem::forget(children_vec);

        LayoutNode {
            size: self.size,
            location: self.location,
            children: children,
        }
    }
}
