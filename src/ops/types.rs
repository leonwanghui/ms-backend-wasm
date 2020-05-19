use std::boxed::Box;

pub enum OpType {
    Add,
    Mul,
    Argmax,
    Reshape,
    EqualCount,
}

pub enum OpStatus {
    Succeed,
    InitFailed,
    LaunchFailed,
}

pub trait OpInfo {
    fn init(&mut self) -> OpStatus;

    fn launch(&self, inputs: *const Vec<Box<Address>>, outputs: *mut Vec<Box<Address>>)
        -> OpStatus;
}

pub struct Address {
    pub addr: *const i32,
    pub size: i32,
}

impl Address {
    pub fn new(addr: *const i32, size: i32) -> Self {
        Address {
            addr: addr,
            size: size,
        }
    }
}
