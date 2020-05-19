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
use types::{OpInfo, OpType};

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
