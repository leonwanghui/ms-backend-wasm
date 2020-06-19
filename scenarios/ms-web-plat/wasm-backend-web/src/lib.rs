#[macro_use]
extern crate serde_derive;

mod ops;
use ops::types::Status;
mod utils;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run(op_type: i32, data_type: i32, value: String) -> JsValue {
    let (stat, dtype) = ops::parse_data_type(data_type as usize);
    if stat != Status::Succeed {
        return JsValue::null();
    }

    let inputs = utils::decode_inputs(value, data_type);
    let (a_shape, b_shape) = ops::parse_inputs_shape(&inputs);
    let (a_dim_size, b_dim_size) = ops::parse_inputs_dim_size(&inputs);
    let inputs_data = ops::parse_inputs_data(&inputs);

    let mut op_instance = ops::operator_instantiate(op_type as usize);
    if op_instance.init(dtype, a_shape, a_dim_size, b_shape, b_dim_size) != Status::Succeed {
        return JsValue::null();
    };

    let (stat, outputs) = op_instance.launch(inputs_data);
    if stat != Status::Succeed {
        return JsValue::null();
    }

    utils::encode_outputs(outputs)
}
