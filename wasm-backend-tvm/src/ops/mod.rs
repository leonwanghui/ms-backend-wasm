mod add;
use add::TVMAddOp;
mod sub;
use sub::TVMSubOp;
pub mod types;
use types::*;

use std::boxed::Box;

pub fn operator_instantiate(op_type: i32) -> Box<dyn Operator> {
    match OpType::from(op_type) {
        OpType::Add => Box::new(TVMAddOp::new()),
        OpType::Sub => Box::new(TVMSubOp::new()),
    }
}

pub fn validate_inputs(inputs: &Vec<Tensor>) -> Status {
    if (inputs.len() == 3
        && !(inputs[0].dtype() == inputs[1].dtype() && inputs[0].dtype() == inputs[2].dtype()))
        || (inputs.len() == 2 && inputs[0].dtype() != inputs[1].dtype())
    {
        println!("The dtype of inputs and outputs is not equal!");
        Status::ValidateFailed
    } else {
        Status::Succeed
    }
}

pub fn parse_inputs_shape(inputs: &Vec<Tensor>) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    if inputs.len() == 3 {
        (inputs[0].shape(), inputs[1].shape(), inputs[2].shape())
    } else {
        (inputs[0].shape(), inputs[1].shape(), Vec::new())
    }
}

pub fn parse_inputs_tensor(inputs: &Vec<Tensor>) -> (Vec<Tensor>, Tensor) {
    if inputs.len() == 3 {
        (
            vec![inputs[0].clone(), inputs[1].clone()],
            inputs[2].clone(),
        )
    } else {
        (vec![inputs[0].clone()], inputs[1].clone())
    }
}
