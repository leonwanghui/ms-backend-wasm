#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod ops;
use ops::types::OpStatus;

#[no_mangle]
pub extern "C" fn run(
    op_type: i32,
    data_type: i32,
    dim_size: i32,
    shape_1d_size: i32,
    shape_2d_size: i32,
    shape_3d_size: i32,
    in_addr: i32,
    in_size: i32,
    out_addr: i32,
) -> i32 {
    let mut op_instance = ops::operator_instantiate(op_type);
    let inputs = ops::load_inputs(in_addr, in_size);

    let shape_slice = vec![
        shape_1d_size as usize,
        shape_2d_size as usize,
        shape_3d_size as usize,
    ];
    if op_instance.init(data_type, shape_slice, dim_size as usize) != OpStatus::Succeed {
        return 0;
    };

    let (stat, outputs) = op_instance.launch(inputs);
    if stat != OpStatus::Succeed {
        return 0;
    }

    ops::store_outputs(out_addr, outputs)
}
