use super::types::{Address, OpInfo, OpStatus};
use nalgebra::Vector3;
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
        let input_vec = unsafe { inputs.as_ref().unwrap() };
        let output_vec = unsafe { outputs.as_ref().unwrap() };

        if input_vec.len() != 1 || output_vec.len() != 1 {
            println!("Inputs and outputs vector length should be 1!");
            return OpStatus::LaunchFailed;
        }
        let vec = Vector3::new(0, 0, 0);

        for i in 0..input_vec[0].size {
            unsafe {
                vec.push(ptr::read(input_vec[0].addr.offset(i as isize)));
            }
        }

        let res_tuple = vec.argmax();
        let result = (res_tuple.0 as i32, res_tuple.1);
        println!("ArgmaxOp result is {:?}", result);
        let result_ptr: *const i32 = &result.0;
        let output_addr = Box::new(Address::new(result_ptr, 1));
        let mut output_vec = Vec::new();
        output_vec.push(output_addr);
        unsafe {
            *outputs = output_vec;
        }
        println!("ArgmaxOp run success!");
        OpStatus::Succeed
    }
}
