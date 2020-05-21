use std::boxed::Box;

#[derive(PartialEq)]
pub enum OpType {
    Add,
    Mul,
    Argmax,
    EqualCount,
}

#[derive(PartialEq)]
pub enum OpStatus {
    Succeed,
    InitFailed,
    LaunchFailed,
}

pub trait OpInfo {
    fn init(&mut self) -> OpStatus;

    fn launch(&self, inputs: Vec<Box<Vec<i32>>>) -> (OpStatus, Vec<Box<Vec<i32>>>);
}
