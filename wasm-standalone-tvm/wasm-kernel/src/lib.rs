/*
 * Licensed to the Apache Software Foundation (ASF) under one
 * or more contributor license agreements.  See the NOTICE file
 * distributed with this work for additional information
 * regarding copyright ownership.  The ASF licenses this file
 * to you under the Apache License, Version 2.0 (the
 * "License"); you may not use this file except in compliance
 * with the License.  You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing,
 * software distributed under the License is distributed on an
 * "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
 * KIND, either express or implied.  See the License for the
 * specific language governing permissions and limitations
 * under the License.
 */

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tvm_runtime;

mod ops;
use ops::types::Status;
mod utils;

#[no_mangle]
pub extern "C" fn run(op_type: i32, wasm_addr: i32, in_size: i32) -> i32 {
    let inputs = utils::load_inputs(wasm_addr, in_size as usize);
    if ops::validate_inputs(&inputs) != Status::Succeed {
        return 0i32;
    }

    let op_instance = ops::operator_instantiate(op_type);
    let (a_shape, b_shape, c_shape) = ops::parse_inputs_shape(&inputs);
    if op_instance.init(a_shape, b_shape, c_shape) != Status::Succeed {
        return 0i32;
    };

    let (in_tensors, out_tensor) = ops::parse_inputs_tensor(&inputs);
    let (stat, output) = op_instance.launch(in_tensors, out_tensor);
    if stat != Status::Succeed {
        return 0i32;
    }

    let out_size = utils::store_output(wasm_addr, output);
    out_size as i32
}
