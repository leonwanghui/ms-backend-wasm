pub mod ops;
use ops::parse_optype;

#[no_mangle]
pub extern "C" fn launch(op_type: i32) -> i32 {
    let mut op = parse_optype(op_type);
    op.init();
    op.run()
}
