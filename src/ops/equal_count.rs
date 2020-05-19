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
        unsafe {
            if inputs.as_ref().unwrap().len() != 1 && outputs.as_ref().unwrap().len() != 1 {
                println!("Inputs outputs size not support");
                return OpStatus::LaunchFailed;
            }
        }

        let mut left = Vec::default();
        let mut right = Vec::default();
        let mut num = 0;
        unsafe {
            for i in 0..inputs.as_ref().unwrap()[0].size {
                left.push(ptr::read(
                    inputs.as_ref().unwrap()[0].addr.offset(i as isize),
                ));
            }
            for i in 0..inputs.as_ref().unwrap()[0].size {
                right.push(ptr::read(
                    inputs.as_ref().unwrap()[0].addr.offset(i as isize),
                ));
            }
            for i in 0..inputs.as_ref().unwrap()[0].size {
                if left[i as usize] == right[i as usize] {
                    num += 1;
                }
            }
            let output_addr = Box::new(Address::new(&mut num, 1));
            // outputs.as_ref().unwrap().push(output_addr);
        }
        println!("EqualCountOp result is {}", num);
        println!("EqualCountOp run success!");
        OpStatus::Succeed
    }
}
