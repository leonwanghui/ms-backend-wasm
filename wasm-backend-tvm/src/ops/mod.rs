mod add;
use add::TVMAddOp;
pub mod types;
use types::*;

use std::boxed::Box;

pub fn operator_instantiate(op_type: i32) -> Box<dyn Operator> {
    match OpType::from(op_type) {
        OpType::Add => Box::new(TVMAddOp::new()),
    }
}

pub fn parse_inputs_dtype(inputs: &Vec<Tensor>) -> (Status, DataType) {
    if inputs.len() == 3
        && (inputs[0].dtype() != inputs[1].dtype() || inputs[0].dtype() != inputs[2].dtype())
    {
        println!("The dtype of inputs is not equal!");
        (Status::ParseFailed, DataType::FP32)
    } else if inputs.len() == 2 && inputs[0].dtype() != inputs[1].dtype() {
        println!("The dtype of inputs and outputs is not equal!");
        (Status::ParseFailed, DataType::FP32)
    } else {
        (Status::Succeed, inputs[0].dtype())
    }
}

pub fn parse_inputs_shape(inputs: &Vec<Tensor>) -> (Vec<usize>, Vec<usize>) {
    if inputs.len() == 3 {
        (inputs[0].shape(), inputs[1].shape())
    } else {
        (inputs[0].shape(), Vec::new())
    }
}

pub fn parse_inputs_tensor(inputs: &Vec<Tensor>) -> (Vec<*mut DLTensor>, *mut DLTensor) {
    if inputs.len() == 3 {
        (
            vec![
                Box::into_raw(Box::from(inputs[0].as_dltensor())),
                Box::into_raw(Box::from(inputs[1].as_dltensor())),
            ],
            Box::into_raw(Box::from(inputs[2].as_dltensor())),
        )
    } else {
        (
            vec![Box::into_raw(Box::from(inputs[0].as_dltensor()))],
            Box::into_raw(Box::from(inputs[1].as_dltensor())),
        )
    }
}
