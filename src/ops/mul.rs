use super::types::{DataType, OpStatus, Operator, Tensor};
use ndarray::Array1;
use std::boxed::Box;

pub struct MulOp {
    data_type: Option<DataType>,
    shape: Vec<usize>,
    dim_size: usize,
}

impl MulOp {
    pub fn new() -> MulOp {
        MulOp {
            data_type: None,
            shape: Vec::new(),
            dim_size: 0,
        }
    }
}

impl Operator for MulOp {
    fn init(&mut self, data_type: i32, shape: Vec<usize>, dim_size: usize) -> OpStatus {
        match data_type {
            0 => {
                println!("MulOp doesn't support boolean type as inputs!");
                return OpStatus::InitFailed;
            }
            1 => {
                self.data_type = Some(DataType::NumericType);
            }
            2 => {
                self.data_type = Some(DataType::OneDArrayType);
            }
            3 => {
                self.data_type = Some(DataType::TwoDArrayType);
            }
            4 => {
                self.data_type = Some(DataType::ThreeDArrayType);
            }
            _ => {
                println!("Unknown data type provided!");
                return OpStatus::InitFailed;
            }
        }
        self.shape = shape;
        self.dim_size = dim_size;
        println!("MulOp init success!");
        OpStatus::Succeed
    }
    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (OpStatus, Vec<Box<Tensor>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (OpStatus::LaunchFailed, vec![Box::new(Tensor::from(false))]);
        }

        let mut output_vec = Vec::new();
        match self.data_type {
            Some(DataType::NumericType) => {
                let left = inputs[0].cast_f32();
                let right = inputs[1].cast_f32();
                let result = left * right;
                output_vec.push(Box::new(Tensor::from(result)));
            }
            Some(DataType::OneDArrayType) => {
                let left_vec = inputs[0].cast_1d_array();
                let right_vec = inputs[1].cast_1d_array();
                let left = Array1::from(left_vec);
                let right = Array1::from(right_vec);
                let result = left.dot(&right);
                output_vec.push(Box::new(Tensor::from(vec![result])));
            }
            Some(DataType::TwoDArrayType) => {}
            Some(DataType::ThreeDArrayType) => {}
            _ => {
                println!("Unknown data type provided!");
                return (OpStatus::LaunchFailed, vec![Box::new(Tensor::from(false))]);
            }
        }

        println!("MulOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
