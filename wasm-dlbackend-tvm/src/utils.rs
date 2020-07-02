use super::types::*;
use serde_json;
use std::ptr;

pub fn load_input(in_addr: i32, in_size: usize) -> Tensor {
    let in_addr = in_addr as *mut u8;

    let mut data_vec = Vec::new();
    for i in 0..in_size {
        data_vec.push(unsafe { ptr::read(in_addr.offset(i as isize)) });
    }
    let input: Tensor = serde_json::from_slice(&data_vec).unwrap();

    input
}

pub fn store_output(out_addr: i32, output: Tensor) -> usize {
    let out_addr = out_addr as *mut u8;

    let data_vec = serde_json::to_vec(&output).unwrap();
    let data_size = data_vec.len();
    for i in 0..data_size {
        unsafe {
            ptr::write(out_addr.offset(i as isize), *data_vec.get(i).unwrap());
        }
    }

    data_size
}
