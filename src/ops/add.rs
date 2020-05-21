use super::types::{OpInfo, OpStatus};
use ndarray::Array1;
use std::boxed::Box;

pub struct AddOp {}

impl AddOp {
    pub fn new() -> AddOp {
        AddOp {}
    }
}

impl OpInfo for AddOp {
    fn init(&mut self) -> OpStatus {
        println!("AddOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Vec<i32>>>) -> (OpStatus, Vec<Box<Vec<i32>>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (OpStatus::LaunchFailed, vec![Box::new(Vec::new())]);
        }

        let left = Array1::from(inputs[0].to_vec());
        let right = Array1::from(inputs[1].to_vec());
        let result = &left + &right;

        let mut output_vec = Vec::new();
        output_vec.push(Box::new(result.to_vec()));
        println!("AddOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
