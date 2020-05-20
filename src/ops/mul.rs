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
        let input_vec = unsafe { inputs.as_ref().unwrap() };
        let output_vec = unsafe { outputs.as_ref().unwrap() };

        if input_vec.len() != 2 || output_vec.len() != 1 {
            println!("Inputs vector length should be 2, outputs vector length should be 1!");
            return OpStatus::LaunchFailed;
        }

        let mut vec = Vec::new();
        for i in 0..input_vec[0].size {
            unsafe {
                vec.push(ptr::read(input_vec[0].addr.offset(i as isize)));
            }
        }
        let left = Array1::from(vec);
        let mut vec = Vec::new();

        for i in 0..input_vec[1].size {
            unsafe {
                vec.push(ptr::read(input_vec[1].addr.offset(i as isize)));
            }
        }
        let right = Array1::from(vec);
        let result = right.dot(&left);
        println!("MulOp result is {}", result);

        let output_addr = Box::new(Address::new(&result, 1));
        let mut output_vec = Vec::new();
        output_vec.push(output_addr);
        unsafe {
            *outputs = output_vec;
        }
        println!("MulOp run success!");
        OpStatus::Succeed
    }
}
