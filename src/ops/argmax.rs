use super::types::{DataType, OpStatus, Operator, Tensor};
use std::boxed::Box;

pub struct ArgmaxOp {
    data_type: Option<DataType>,
    shape: Vec<usize>,
    dim_size: usize,
}

impl ArgmaxOp {
    pub fn new() -> ArgmaxOp {
        ArgmaxOp {
            data_type: None,
            shape: Vec::new(),
            dim_size: 0,
        }
    }
}

impl Operator for ArgmaxOp {
    fn init(&mut self, data_type: i32, shape: Vec<usize>, dim_size: usize) -> OpStatus {
        match data_type {
            0 => {
                println!("ArgmaxOp doesn't support boolean type as inputs!");
                return OpStatus::InitFailed;
            }
            1 => {
                println!("ArgmaxOp doesn't support numeric type as inputs!");
                return OpStatus::InitFailed;
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
        println!("EqualCountOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (OpStatus, Vec<Box<Tensor>>) {
        if inputs.len() == 0 {
            println!("Inputs vector length should not be zero!");
            return (OpStatus::LaunchFailed, vec![Box::new(Tensor::from(false))]);
        }

        let mut index = 0;
        let mut max_arg = 0.0f32;
        match self.data_type {
            Some(DataType::OneDArrayType) => {
                let input_vec = inputs[0].cast_1d_array();
                max_arg = input_vec[0];

                for i in 0..input_vec.len() {
                    if input_vec[i] > max_arg {
                        max_arg = input_vec[i];
                        index = i;
                    }
                }
            }
            Some(DataType::TwoDArrayType) => {}
            Some(DataType::ThreeDArrayType) => {}
            _ => {
                println!("Unknown data type provided!");
                return (OpStatus::LaunchFailed, vec![Box::new(Tensor::from(false))]);
            }
        }

        let mut output_vec = Vec::new();
        output_vec.push(Box::new(Tensor::from(vec![max_arg, index as f32])));
        println!("ArgmaxOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
