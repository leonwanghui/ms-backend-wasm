use super::ops::types::*;
use serde_json;
use std::ptr;

pub fn load_inputs(in_addr: i32, in_size: usize) -> Vec<Tensor> {
    let in_addr = in_addr as *mut u8;

    let mut data_vec = Vec::new();
    for i in 0..in_size {
        data_vec.push(unsafe { ptr::read(in_addr.offset(i as isize)) });
    }
    let inputs: Vec<Tensor> = serde_json::from_slice(&data_vec).unwrap();

    inputs
}

pub fn store_outputs(out_addr: i32, outputs: DLTensor) -> usize {
    let out_addr = out_addr as *mut u8;

    let result: Tensor = outputs.into();
    let data_vec = serde_json::to_vec(&result).unwrap();
    let data_size = data_vec.len();
    for i in 0..data_size {
        unsafe {
            ptr::write(out_addr.offset(i as isize), *data_vec.get(i).unwrap());
        }
    }

    data_size
}
