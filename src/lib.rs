pub mod ops;
use ops::parse_optype;
use ops::types::Address;

#[no_mangle]
pub extern "C" fn launch(op_type: i32) -> i32 {
    let mut op = parse_optype(op_type);

    let data = [1, 2, 3];
    let data_ptr = &data as *const i32;
    let mut inputs = Vec::default();
    inputs.push(Box::new(Address::new(data_ptr, 3)));
    inputs.push(Box::new(Address::new(data_ptr, 3)));

    let mut outputs = Vec::default();
    outputs.push(Box::new(Address::new(data_ptr, 3)));

    op.init();
    op.launch(&inputs, &mut outputs) as i32
}
