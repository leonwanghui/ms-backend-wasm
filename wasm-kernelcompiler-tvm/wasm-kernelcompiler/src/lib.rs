#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tvm_runtime;

mod ops;
use ops::types::Status;
mod utils;

#[no_mangle]
pub extern "C" fn run(op_type: i32, wasm_addr: i32, in_size: i32) -> i32 {
    let inputs = utils::load_inputs(wasm_addr, in_size as usize);
    if ops::validate_inputs(&inputs) != Status::Succeed {
        return 0i32;
    }

    let op_instance = ops::operator_instantiate(op_type);
    let (a_shape, b_shape, c_shape) = ops::parse_inputs_shape(&inputs);
    if op_instance.init(a_shape, b_shape, c_shape) != Status::Succeed {
        return 0i32;
    };

    let (in_tensors, out_tensor) = ops::parse_inputs_tensor(&inputs);
    let (stat, output) = op_instance.launch(in_tensors, out_tensor);
    if stat != Status::Succeed {
        return 0i32;
    }

    let out_size = utils::store_output(wasm_addr, output);
    out_size as i32
}
