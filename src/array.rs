#[repr(C)]
#[derive(Debug)]
pub struct Array<T> {
    pub pointer: *mut T,
    pub length: usize,
    pub capacity: usize,
}
