use super::types::{Address, OpInfo, OpStatus};
use ndarray::Array1;
use std::boxed::Box;
use std::ptr;

pub struct MulOp {}

impl MulOp {
    pub fn new() -> MulOp {
        MulOp {}
    }
}

impl OpInfo for MulOp {
    fn init(&mut self) -> OpStatus {
        println!("MulOp init success!");
        OpStatus::Succeed
    }

    fn launch(
        &self,
        inputs: *const Vec<Box<Address>>,
        outputs: *mut Vec<Box<Address>>,
    ) -> OpStatus {
        unsafe {
            if inputs.as_ref().unwrap().len() != 2 && outputs.as_ref().unwrap().len() != 1 {
                println!("Inputs outputs size not support");
                return OpStatus::LaunchFailed;
            }
        }

        let mut vec = Vec::default();
        unsafe {
            for i in 0..inputs.as_ref().unwrap()[0].size {
                vec.push(ptr::read(
                    inputs.as_ref().unwrap()[0].addr.offset(i as isize),
                ));
            }
        }
        let left = Array1::from(vec);
        let mut vec = Vec::default();
        unsafe {
            for i in 0..inputs.as_ref().unwrap()[1].size {
                vec.push(ptr::read(
                    inputs.as_ref().unwrap()[1].addr.offset(i as isize),
                ));
            }
        }
        let right = Array1::from(vec);

        let result = right.dot(&left);
        println!("MulOp result is {}", result);
        println!("MulOp run success!");
        // let output_addr = Box::new(Address::new(result.as_ptr(), result.len() as i32));
        // outputs.push(output_addr);
        OpStatus::Succeed
    }
}
