use super::types::{Address, OpInfo, OpStatus};
use nalgebra::Vector2;
use std::boxed::Box;
use std::ptr;

pub struct ArgmaxOp {}

impl ArgmaxOp {
    pub fn new() -> ArgmaxOp {
        ArgmaxOp {}
    }
}

impl OpInfo for ArgmaxOp {
    fn init(&mut self) -> OpStatus {
        println!("ArgmaxOp init success!");
        OpStatus::Succeed
    }

    fn launch(
        &self,
        inputs: *const Vec<Box<Address>>,
        outputs: *mut Vec<Box<Address>>,
    ) -> OpStatus {
        unsafe {
            if inputs.as_ref().unwrap().len() != 1 && outputs.as_ref().unwrap().len() != 1 {
                println!("Inputs outputs size not support");
                return OpStatus::LaunchFailed;
            }
        }

        let vec = Vector2::new(0, 0);
        unsafe {
            for i in 0..inputs.as_ref().unwrap()[0].size {
                vec.push(ptr::read(
                    inputs.as_ref().unwrap()[0].addr.offset(i as isize),
                ));
            }
        }
        let result = vec.argmax();
        println!("ArgmaxOp result is {:?}", result);
        println!("ArgmaxOp run success!");
        OpStatus::Succeed
    }
}
