use super::types::{DataType, OpStatus, Operator, Tensor};
use std::boxed::Box;

pub struct EqualCountOp {
    data_type: Option<DataType>,
    shape: Vec<usize>,
    dim_size: usize,
}

impl EqualCountOp {
    pub fn new() -> EqualCountOp {
        EqualCountOp {
            data_type: None,
            shape: Vec::new(),
            dim_size: 0,
        }
    }
}

impl Operator for EqualCountOp {
    fn init(&mut self, data_type: i32, shape: Vec<usize>, dim_size: usize) -> OpStatus {
        match data_type {
            0 => {
                self.data_type = Some(DataType::BooleanType);
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
        println!("EqualCountOp init success!");
        OpStatus::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (OpStatus, Vec<Box<Tensor>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (OpStatus::LaunchFailed, vec![Box::new(Tensor::from(false))]);
        }

        let mut num = 0;
        match self.data_type {
            Some(DataType::BooleanType) => {
                let left = inputs[0].cast_bool();
                let right = inputs[1].cast_bool();
                if left == right {
                    num += 1
                };
            }
            Some(DataType::NumericType) => {
                let left = inputs[0].cast_f32();
                let right = inputs[0].cast_f32();
                if left == right {
                    num += 1
                };
            }
            Some(DataType::OneDArrayType) => {
                let left = inputs[0].cast_1d_array();
                let right = inputs[0].cast_1d_array();
                if left.len() != right.len() {
                    println!("Inputs size not equal!");
                    return (OpStatus::LaunchFailed, vec![Box::new(Tensor::from(false))]);
                }

                for i in 0..left.len() {
                    if left[i as usize] == right[i as usize] {
                        num += 1;
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
        output_vec.push(Box::new(Tensor::from(num as usize)));
        println!("EqualCountOp run success!");
        (OpStatus::Succeed, output_vec)
    }
}
