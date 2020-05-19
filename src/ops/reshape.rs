use super::types::{Address, OpInfo, OpStatus};
use std::boxed::Box;
use std::ptr;

pub struct ReshapeOp {}

impl ReshapeOp {
    pub fn new() -> ReshapeOp {
        ReshapeOp {}
    }
}

impl OpInfo for ReshapeOp {
    fn init(&mut self) -> OpStatus {
        println!("ReshapeOp init success!");
        OpStatus::Succeed
    }

    fn launch(
        &self,
        inputs: *const Vec<Box<Address>>,
        outputs: *mut Vec<Box<Address>>,
    ) -> OpStatus {
        unsafe {
            if inputs.as_ref().unwrap()[0].size != outputs.as_ref().unwrap()[0].size {
                return OpStatus::LaunchFailed;
            }
            if inputs.as_ref().unwrap()[0].addr == outputs.as_ref().unwrap()[0].addr {
                return OpStatus::Succeed;
            }

            let mem_bits = outputs.as_ref().unwrap()[0].size;
            let mut ptr_ref = *outputs.as_ref().unwrap()[0].addr;
            let ptr = &mut ptr_ref;
            ptr::copy(inputs.as_ref().unwrap()[0].addr, ptr, mem_bits as usize);
        }
        println!("ReshapeOp run success!");
        OpStatus::Succeed
    }
}
