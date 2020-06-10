#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tvm_runtime;

pub mod ops;
use ops::types::Status;
mod utils;

extern "C" {
    static __tvm_module_ctx: i32;
}

#[no_mangle]
unsafe fn __get_tvm_module_ctx() -> i32 {
    // Refer a symbol in the libtest_wasm32.a to make sure that the link of the
    // library is not optimized out.
    __tvm_module_ctx
}

#[no_mangle]
pub extern "C" fn run(op_type: i32, in_addr: i32, in_size: i32, out_addr: i32) -> i32 {
    let inputs = utils::load_inputs(in_addr, in_size as usize);
    let (stat, dtype) = ops::parse_inputs_dtype(&inputs);
    if stat != Status::Succeed {
        return 0i32;
    };
    let (a_shape, b_shape) = ops::parse_inputs_shape(&inputs);
    let (in_tensor, out_tensor) = ops::parse_inputs_tensor(&inputs);

    let mut op_instance = ops::operator_instantiate(op_type);
    if op_instance.init(dtype, a_shape, b_shape) != Status::Succeed {
        return 0i32;
    };

    let stat = op_instance.launch(in_tensor, out_tensor);
    if stat != Status::Succeed {
        return 0i32;
    }

    let out_size = utils::store_outputs(out_addr, unsafe { *out_tensor });
    out_size as i32
}
