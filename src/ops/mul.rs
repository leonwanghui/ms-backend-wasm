use super::types::{NumberType, OpStatus, Operator};
use super::utils;
use ndarray::Array1;
use std::boxed::Box;

pub struct MulOp {
    num_type: Option<NumberType>,
}

impl MulOp {
    pub fn new() -> MulOp {
        MulOp { num_type: None }
    }
}

impl Operator for MulOp {
    fn init(&mut self, number_type: NumberType) -> OpStatus {
        self.num_type = Some(number_type);
        println!("MulOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Vec<NumberType>>>) -> (OpStatus, Vec<Box<Vec<NumberType>>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (OpStatus::LaunchFailed, Vec::new());
        }

        let mut output_vec = Vec::new();
        if self.num_type == Some(NumberType::FP32(1.0f32)) {
            let left_vec = utils::vec_num_type_fp32_to_f32(inputs[0].to_vec());
            let right_vec = utils::vec_num_type_fp32_to_f32(inputs[1].to_vec());
            let left = Array1::from(left_vec);
            let right = Array1::from(right_vec);
            let result = right.dot(&left);
            output_vec.push(Box::new(utils::vec_f32_to_num_type_fp32(vec![result])));
        } else if self.num_type == Some(NumberType::INT8(1i8)) {
            let left_vec = utils::vec_num_type_int8_to_i8(inputs[0].to_vec());
            let right_vec = utils::vec_num_type_int8_to_i8(inputs[1].to_vec());
            let left = Array1::from(left_vec);
            let right = Array1::from(right_vec);
            let result = right.dot(&left);
            output_vec.push(Box::new(utils::vec_i8_to_num_type_int8(vec![result])));
        } else {
            return (OpStatus::LaunchFailed, Vec::new());
        }

        println!("MulOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
