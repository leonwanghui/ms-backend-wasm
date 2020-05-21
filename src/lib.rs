pub mod ops;

use ops::types::OpStatus;
use ops::{load_inputs, parse_optype, store_outputs};

#[no_mangle]
pub extern "C" fn run(op_type: i32, in_addr: i32, in_size: i32, out_addr: i32) -> i32 {
    let mut op = parse_optype(op_type);

    let inputs = load_inputs(in_addr, in_size);

    if op.init() != OpStatus::Succeed {
        return 0;
    };

    let (stat, outputs) = op.launch(inputs);
    if stat != OpStatus::Succeed {
        return 0;
    }

    store_outputs(out_addr, outputs)
}
