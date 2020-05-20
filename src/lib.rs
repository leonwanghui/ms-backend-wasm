pub mod ops;
use ops::{parse_inputs_outputs, parse_optype};

#[no_mangle]
pub extern "C" fn run(
    op_type: i32,
    in_l_addr: i32,
    in_l_size: i32,
    in_r_addr: i32,
    in_r_size: i32,
    out_addr: i32,
    out_size: i32,
) -> i32 {
    let mut op = parse_optype(op_type);

    let (inputs, mut outputs) = parse_inputs_outputs(
        in_l_addr, in_l_size, in_r_addr, in_r_size, out_addr, out_size,
    );

    op.init();
    op.launch(&inputs, &mut outputs) as i32
}
