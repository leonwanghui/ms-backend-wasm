use super::ops::types::*;
use serde_json::Value;
use serde_json;
use wasm_bindgen::JsValue;

pub fn decode_inputs(val_str: String, data_type: i32) -> Vec<TensorInput> {
    let val: Value = serde_json::from_str(&val_str).unwrap();
    let val_vec = val.as_array().unwrap();
    let mut input_data: Vec<TensorInput> = Vec::new();

    for i in 0..val_vec.len() {
        let data_val = val_vec[i]["input-data"].as_array().unwrap();
        let shape_val = val_vec[i]["shape"].as_array().unwrap();
        let dim_size_val = val_vec[i]["dim-size"].as_u64().unwrap();
        let data: Tensor = if data_type == 0
        /* If data type is FP32 */
        {
            let mut data_vec: Vec<f32> = Vec::new();
            for i in 0..data_val.len() {
                data_vec.push(data_val[i].as_f64().unwrap() as f32);
            }
            Tensor::from(data_vec)
        } else
        /* If data type is INT8 */
        {
            let mut data_vec: Vec<i8> = Vec::new();
            for i in 0..data_val.len() {
                data_vec.push(data_val[i].as_i64().unwrap() as i8);
            }
            Tensor::from(data_vec)
        };
        let mut shape: Vec<usize> = Vec::new();
        for i in 0..shape_val.len() {
            shape.push(shape_val[i].as_u64().unwrap() as usize);
        }
        input_data.push(TensorInput::new(data, &shape, dim_size_val as usize));
    }
    input_data
}

pub fn encode_outputs(outputs: Vec<Box<TensorResult>>) -> JsValue {
    JsValue::from_serde(&outputs[0]).unwrap()
}
