// use super::types::NumberType;

// pub fn vec_num_type_fp32_to_f32(inputs: Vec<NumberType>) -> Vec<f32> {
//     let mut outputs: Vec<f32> = Vec::new();
//     for i in 0..inputs.len() {
//         outputs.push(inputs.get(i).unwrap().cast_f32());
//     }

//     outputs
// }

// pub fn vec_num_type_int8_to_i8(inputs: Vec<NumberType>) -> Vec<i8> {
//     let mut outputs: Vec<i8> = Vec::new();
//     for i in 0..inputs.len() {
//         outputs.push(inputs.get(i).unwrap().cast_i8());
//     }

//     outputs
// }

// pub fn vec_f32_to_num_type_fp32(inputs: Vec<f32>) -> Vec<NumberType> {
//     let mut outputs: Vec<NumberType> = Vec::new();
//     for i in 0..inputs.len() {
//         outputs.push(NumberType::from(inputs[i]));
//     }

//     outputs
// }

// pub fn vec_i8_to_num_type_int8(inputs: Vec<i8>) -> Vec<NumberType> {
//     let mut outputs: Vec<NumberType> = Vec::new();
//     for i in 0..inputs.len() {
//         outputs.push(NumberType::from(inputs[i]));
//     }

//     outputs
// }
