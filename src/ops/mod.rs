mod add;
mod argmax;
mod equal_count;
mod mul;
mod reshape;
pub mod types;

use add::AddOp;
use argmax::ArgmaxOp;
use equal_count::EqualCountOp;
use mul::MulOp;
use reshape::ReshapeOp;
use std::boxed::Box;
use types::{Address, OpInfo, OpType};

pub fn parse_optype(op_type: i32) -> Box<dyn OpInfo> {
    if op_type == OpType::Add as i32 {
        Box::new(AddOp::new())
    } else if op_type == OpType::Mul as i32 {
        Box::new(MulOp::new())
    } else if op_type == OpType::Argmax as i32 {
        Box::new(ArgmaxOp::new())
    } else if op_type == OpType::EqualCount as i32 {
        Box::new(EqualCountOp::new())
    } else if op_type == OpType::Reshape as i32 {
        Box::new(ReshapeOp::new())
    } else {
        Box::new(AddOp::new())
    }
}

pub fn parse_inputs_outputs(
    in_l_addr: i32,
    in_l_size: i32,
    in_r_addr: i32,
    in_r_size: i32,
    out_addr: i32,
    out_size: i32,
) -> (Vec<Box<Address>>, Vec<Box<Address>>) {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();

    if in_l_addr != 0 {
        inputs.push(Box::new(Address::new(in_l_addr as *const i32, in_l_size)));
    }
    if in_r_addr != 0 {
        inputs.push(Box::new(Address::new(in_r_addr as *const i32, in_r_size)));
    }
    if out_addr != 0 {
        outputs.push(Box::new(Address::new(out_addr as *const i32, out_size)));
    }

    (inputs, outputs)
}
