use super::types::{NumberType, OpStatus, Operator};
use std::boxed::Box;

pub struct EqualCountOp {
    num_type: Option<NumberType>,
}

impl EqualCountOp {
    pub fn new() -> EqualCountOp {
        EqualCountOp { num_type: None }
    }
}

impl Operator for EqualCountOp {
    fn init(&mut self, number_type: NumberType) -> OpStatus {
        self.num_type = Some(number_type);
        println!("EqualCountOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Vec<NumberType>>>) -> (OpStatus, Vec<Box<Vec<NumberType>>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (OpStatus::LaunchFailed, Vec::new());
        }
        if inputs[0].len() != inputs[1].len() {
            println!("Inputs size not equal!");
            return (OpStatus::LaunchFailed, Vec::new());
        }

        let mut num = 0;
        for i in 0..inputs[0].len() {
            if inputs[0][i as usize] == inputs[1][i as usize] {
                num += 1;
            }
        }

        let mut output_vec = Vec::new();
        output_vec.push(Box::new(vec![NumberType::from(num as usize)]));
        println!("EqualCountOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
