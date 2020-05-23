#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod ops;
use ops::types::Status;

#[no_mangle]
pub extern "C" fn run(
    op_type: i32,
    data_type: i32,
    a_dim_size: i32,
    a_shape_x: i32,
    a_shape_y: i32,
    a_shape_z: i32,
    b_dim_size: i32,
    b_shape_x: i32,
    b_shape_y: i32,
    b_shape_z: i32,
    in_addr: i32,
    in_size: i32,
    out_addr: i32,
) -> i32 {
    let (stat, dtype) = ops::parse_data_type(data_type as usize);
    if stat != Status::Succeed {
        return 0i32;
    }
    let inputs = ops::load_inputs(in_addr, in_size as usize);
    let a_shape = (a_shape_x as usize, a_shape_y as usize, a_shape_z as usize);
    let b_shape = (b_shape_x as usize, b_shape_y as usize, b_shape_z as usize);

    let mut op_instance = ops::operator_instantiate(op_type as usize);
    if op_instance.init(
        dtype,
        a_shape,
        a_dim_size as usize,
        b_shape,
        b_dim_size as usize,
    ) != Status::Succeed
    {
        return 0i32;
    };

    let (stat, outputs) = op_instance.launch(inputs);
    if stat != Status::Succeed {
        return 0i32;
    }

    let out_size = ops::store_outputs(out_addr, outputs);
    out_size as i32
}
