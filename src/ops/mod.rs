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
use types::{NumberType, OpType, Operator};

pub fn operator_instantiate(op_type: i32) -> Box<dyn Operator> {
    if op_type == OpType::Add as i32 {
        Box::new(AddOp::new())
    } else if op_type == OpType::Mul as i32 {
        Box::new(MulOp::new())
    } else if op_type == OpType::Argmax as i32 {
        Box::new(ArgmaxOp::new())
    } else if op_type == OpType::EqualCount as i32 {
        Box::new(EqualCountOp::new())
    } else {
        Box::new(AddOp::new())
    }
}

pub fn parse_num_type(num_type: i32) -> NumberType {
    match num_type {
        0 => NumberType::FP32(1.0f32),
        1 => NumberType::INT8(1i8),
        _ => NumberType::FP32(1.0f32),
    }
}

pub fn load_inputs(in_addr: i32, in_size: i32) -> Vec<Box<Vec<NumberType>>> {
    let in_addr = in_addr as *mut u8;

    let mut data_vec = Vec::new();
    for i in 0..in_size {
        data_vec.push(unsafe { ptr::read(in_addr.offset(i as isize)) });
    }
    let inputs: Vec<Box<Vec<NumberType>>> = serde_json::from_slice(&data_vec).unwrap();

    inputs
}

pub fn store_outputs(out_addr: i32, outputs: Vec<Box<Vec<NumberType>>>) -> i32 {
    let out_addr = out_addr as *mut u8;

    let data_vec = serde_json::to_vec(&outputs).unwrap();
    let data_size = data_vec.len();
    for i in 0..data_size {
        unsafe {
            ptr::write(out_addr.offset(i as isize), *data_vec.get(i).unwrap());
        }
    }

    data_size as i32
}
