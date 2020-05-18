pub enum OpType {
    Add,
    Mul,
    Inverse,
}

pub trait OpInfo {
    fn init(&mut self) -> bool;

    fn run(&self) -> i32;
}
