mod add;
mod argmax;
mod equal_count;
mod mul;
pub mod types;

use add::AddOp;
use argmax::ArgmaxOp;
use equal_count::EqualCountOp;
use mul::MulOp;
use std::boxed::Box;
use types::*;

pub fn operator_instantiate(op_type: usize) -> Box<dyn Operator> {
    if op_type == OpType::Add as usize {
        Box::new(AddOp::new())
    } else if op_type == OpType::Mul as usize {
        Box::new(MulOp::new())
    } else if op_type == OpType::Argmax as usize {
        Box::new(ArgmaxOp::new())
    } else if op_type == OpType::EqualCount as usize {
        Box::new(EqualCountOp::new())
    } else {
        Box::new(AddOp::new())
    }
}

pub fn parse_data_type(dtype: usize) -> (Status, DataType) {
    match dtype {
        0 => (Status::Succeed, DataType::FP32),
        1 => (Status::Succeed, DataType::INT32),
        2 => (Status::Succeed, DataType::INT8),
        _ => {
            println!("Unknown data type provided!");
            (Status::ParseFailed, DataType::FP32)
        }
    }
}

pub fn parse_inputs_shape(inputs: &Vec<TensorWrapper>) -> (Vec<usize>, Vec<usize>) {
    let a_shape: Vec<usize> = match &inputs[0].shape {
        Some(i) => i.to_vec(),
        _ => Vec::new(),
    };
    let b_shape: Vec<usize> = if inputs.len() == 2 {
        let b: Vec<usize> = match &inputs[1].shape {
            Some(i) => i.to_vec(),
            _ => Vec::new(),
        };
        b
    } else {
        Vec::new()
    };

    (a_shape, b_shape)
}

pub fn parse_inputs_dim_size(inputs: &Vec<TensorWrapper>) -> (usize, usize) {
    let a_dim_size: usize = match inputs[0].dim_size {
        Some(i) => i,
        _ => 0,
    };
    let b_dim_size: usize = if inputs.len() == 2 {
        let b = match inputs[1].dim_size {
            Some(i) => i,
            _ => 0,
        };
        b
    } else {
        0
    };

    (a_dim_size, b_dim_size)
}

pub fn parse_inputs_data(inputs: &Vec<TensorWrapper>) -> Vec<Box<Tensor>> {
    let mut inputs_data = Vec::new();

    match &inputs[0].data {
        Some(i) => inputs_data.push(Box::new(i.clone())),
        _ => (),
    }
    if inputs.len() == 2 {
        match &inputs[1].data {
            Some(i) => inputs_data.push(Box::new(i.clone())),
            _ => (),
        }
    }
    inputs_data
}
