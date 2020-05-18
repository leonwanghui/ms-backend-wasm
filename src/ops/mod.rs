mod add;
mod inverse;
mod mul;
mod types;

use add::AddOp;
use inverse::InverseOp;
use mul::MulOp;
use std::boxed::Box;
use types::{OpInfo, OpType};

pub fn parse_optype(op_type: i32) -> Box<dyn OpInfo> {
    if op_type == OpType::Add as i32 {
        Box::new(AddOp::new())
    } else if op_type == OpType::Mul as i32 {
        Box::new(MulOp::new())
    } else if op_type == OpType::Inverse as i32 {
        Box::new(InverseOp::new())
    } else {
        Box::new(AddOp::new())
    }
}
