use super::types::{Address, OpInfo, OpStatus};
use std::boxed::Box;
use std::ptr;

pub struct EqualCountOp {}

impl EqualCountOp {
    pub fn new() -> EqualCountOp {
        EqualCountOp {}
    }
}

impl OpInfo for EqualCountOp {
    fn init(&mut self) -> OpStatus {
        println!("EqualCountOp init success!");
        OpStatus::Succeed
    }

    fn launch(
        &self,
        inputs: *const Vec<Box<Address>>,
        outputs: *mut Vec<Box<Address>>,
    ) -> OpStatus {
        let input_vec = unsafe { inputs.as_ref().unwrap() };

        if input_vec.len() < 2 {
            println!("Inputs vector length should be over 2!");
            return OpStatus::LaunchFailed;
        }
        if input_vec[0].size != input_vec[1].size {
            println!("Inputs size not equal!");
            return OpStatus::LaunchFailed;
        }

        let mut left = Vec::new();
        let mut right = Vec::new();
        let mut num = 0;
        for i in 0..input_vec[0].size {
            unsafe {
                left.push(ptr::read(input_vec[0].addr.offset(i as isize)));
            }
        }
        for i in 0..input_vec[0].size {
            unsafe {
                right.push(ptr::read(input_vec[0].addr.offset(i as isize)));
            }
        }
        for i in 0..input_vec[0].size {
            if left[i as usize] == right[i as usize] {
                num += 1;
            }
        }
        println!("EqualCountOp result is {}", num);

        let output_addr = Box::new(Address::new(&mut num, 1));
        let mut output_vec = Vec::new();
        output_vec.push(output_addr);
        unsafe {
            *outputs = output_vec;
        }
        println!("EqualCountOp run success!");
        OpStatus::Succeed
    }
}
