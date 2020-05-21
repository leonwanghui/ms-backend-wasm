use super::types::{OpInfo, OpStatus};
use nalgebra::Vector3;
use std::boxed::Box;

pub struct ArgmaxOp {}

impl ArgmaxOp {
    pub fn new() -> ArgmaxOp {
        ArgmaxOp {}
    }
}

impl OpInfo for ArgmaxOp {
    fn init(&mut self) -> OpStatus {
        println!("ArgmaxOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Vec<i32>>>) -> (OpStatus, Vec<Box<Vec<i32>>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 1!");
            return (OpStatus::LaunchFailed, vec![Box::new(Vec::new())]);
        }

        let vec = Vector3::new(1, 2, 3);
        let result = vec.argmax();

        let mut output_vec = Vec::new();
        output_vec.push(Box::new(vec![result.0 as i32, result.1]));
        println!("ArgmaxOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
