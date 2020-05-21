use super::types::{OpInfo, OpStatus};
use std::boxed::Box;

pub struct EqualCountOp {}

impl EqualCountOp {
    pub fn new() -> EqualCountOp {
        EqualCountOp {}
    }
}

impl OpInfo for EqualCountOp {
    fn init(&mut self) -> OpStatus {
        println!("EqualCountOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Vec<i32>>>) -> (OpStatus, Vec<Box<Vec<i32>>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (OpStatus::LaunchFailed, vec![Box::new(Vec::new())]);
        }
        if inputs[0].len() != inputs[1].len() {
            println!("Inputs size not equal!");
            return (OpStatus::LaunchFailed, vec![Box::new(Vec::new())]);
        }

        let mut num = 0;
        for i in 0..inputs[0].len() {
            if inputs[0][i as usize] == inputs[1][i as usize] {
                num += 1;
            }
        }

        let mut output_vec = Vec::new();
        output_vec.push(Box::new(vec![num]));
        println!("EqualCountOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
