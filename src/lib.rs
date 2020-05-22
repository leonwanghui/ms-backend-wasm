#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod ops;
use ops::types::OpStatus;

#[no_mangle]
pub extern "C" fn run(
    op_type: i32,
    num_type: i32,
    in_addr: i32,
    in_size: i32,
    out_addr: i32,
) -> i32 {
    let mut op_instance = ops::operator_instantiate(op_type);
    let number_type = ops::parse_num_type(num_type);
    let inputs = ops::load_inputs(in_addr, in_size);

    if op_instance.init(number_type) != OpStatus::Succeed {
        return 0;
    };

    let (stat, outputs) = op_instance.launch(inputs);
    if stat != OpStatus::Succeed {
        return 0;
    }

    ops::store_outputs(out_addr, outputs)
}
