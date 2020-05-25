#[macro_use]
extern crate serde_derive;

pub mod ops;
use ops::types::Status;
mod utils;

#[no_mangle]
pub extern "C" fn run(
    op_type: i32,
    data_type: i32,
    in_addr: i32,
    in_size: i32,
    out_addr: i32,
) -> i32 {
    let (stat, dtype) = ops::parse_data_type(data_type as usize);
    if stat != Status::Succeed {
        return 0i32;
    }

    let inputs = utils::load_inputs(in_addr, in_size as usize);
    let (a_shape, b_shape) = ops::parse_inputs_shape(&inputs);
    let (a_dim_size, b_dim_size) = ops::parse_inputs_dim_size(&inputs);
    let inputs_data = ops::parse_inputs_data(&inputs);

    let mut op_instance = ops::operator_instantiate(op_type as usize);
    if op_instance.init(dtype, a_shape, a_dim_size, b_shape, b_dim_size) != Status::Succeed {
        return 0i32;
    };

    let (stat, outputs) = op_instance.launch(inputs_data);
    if stat != Status::Succeed {
        return 0i32;
    }

    let out_size = utils::store_outputs(out_addr, outputs);
    out_size as i32
}
