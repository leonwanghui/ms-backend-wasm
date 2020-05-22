use super::types::{NumberType, OpStatus, Operator};
use super::utils;
use nalgebra::DVector;
use std::boxed::Box;

pub struct ArgmaxOp {
    num_type: Option<NumberType>,
}

impl ArgmaxOp {
    pub fn new() -> ArgmaxOp {
        ArgmaxOp { num_type: None }
    }
}

impl Operator for ArgmaxOp {
    fn init(&mut self, number_type: NumberType) -> OpStatus {
        self.num_type = Some(number_type);
        println!("ArgmaxOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Vec<NumberType>>>) -> (OpStatus, Vec<Box<Vec<NumberType>>>) {
        if inputs.len() == 0 {
            println!("Inputs vector length should not be zero!");
            return (OpStatus::LaunchFailed, Vec::new());
        }

        let mut output_vec = Vec::new();
        if self.num_type == Some(NumberType::FP32(1.0f32)) {
            let input_vec = utils::vec_num_type_fp32_to_f32(inputs[0].to_vec());
            let result = DVector::from_vec(input_vec).argmax();
            output_vec.push(Box::new(vec![
                NumberType::from(result.0),
                NumberType::from(result.1),
            ]));
        } else if self.num_type == Some(NumberType::INT8(1i8)) {
            let input_vec = utils::vec_num_type_int8_to_i8(inputs[0].to_vec());
            let result = DVector::from_vec(input_vec).argmax();
            output_vec.push(Box::new(vec![
                NumberType::from(result.0),
                NumberType::from(result.1),
            ]));
        } else {
            return (OpStatus::LaunchFailed, Vec::new());
        }

        println!("ArgmaxOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
