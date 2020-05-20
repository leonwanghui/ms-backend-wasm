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
        let input_vec = unsafe { inputs.as_ref().unwrap() };
        let output_vec = unsafe { outputs.as_ref().unwrap() };

        if input_vec[0].size != output_vec[0].size {
            println!("Inputs outputs size not equal!");
            return OpStatus::LaunchFailed;
        }
        if input_vec[0].addr == output_vec[0].addr {
            println!("Inputs outputs address already equal!");
            return OpStatus::Succeed;
        }

        unsafe {
            let mem_bits = output_vec[0].size;
            let mut ptr_ref = *output_vec[0].addr;
            let ptr = &mut ptr_ref;
            ptr::copy(inputs.as_ref().unwrap()[0].addr, ptr, mem_bits as usize);
        }
        println!("ReshapeOp run success!");
        OpStatus::Succeed
    }
}
