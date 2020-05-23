mod add;
mod argmax;
mod equal_count;
mod mul;
pub mod types;
mod utils;

use add::AddOp;
use argmax::ArgmaxOp;
use equal_count::EqualCountOp;
use mul::MulOp;
use serde_json;
use std::boxed::Box;
use std::ptr;
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
        1 => (Status::Succeed, DataType::INT8),
        _ => {
            println!("Unknown data type provided!");
            (Status::ParseFailed, DataType::FP32)
        }
    }
}

pub fn load_inputs(in_addr: i32, in_size: usize) -> Vec<Box<Tensor>> {
    let in_addr = in_addr as *mut u8;

    let mut data_vec = Vec::new();
    for i in 0..in_size {
        data_vec.push(unsafe { ptr::read(in_addr.offset(i as isize)) });
    }
    let inputs: Vec<Box<Tensor>> = serde_json::from_slice(&data_vec).unwrap();

    inputs
}

pub fn store_outputs(out_addr: i32, outputs: Vec<Box<TensorResult>>) -> usize {
    let out_addr = out_addr as *mut u8;

    let data_vec = serde_json::to_vec(&outputs).unwrap();
    let data_size = data_vec.len();
    for i in 0..data_size {
        unsafe {
            ptr::write(out_addr.offset(i as isize), *data_vec.get(i).unwrap());
        }
    }

    data_size
}
