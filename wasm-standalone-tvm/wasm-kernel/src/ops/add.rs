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

use super::types::*;
use tvm_runtime::{Module as _, SystemLibModule};

extern "C" {
    fn __wasm_call_ctors();
}

pub struct TVMAddOp {}

impl TVMAddOp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Operator for TVMAddOp {
    fn init(&self, a_shape: Vec<i64>, b_shape: Vec<i64>, c_shape: Vec<i64>) -> Status {
        if !((a_shape.len() == b_shape.len()
            && a_shape
                .iter()
                .zip(&b_shape)
                .filter(|&(a, b)| a == b)
                .count()
                == a_shape.len())
            && (b_shape.len() == c_shape.len()
                && b_shape
                    .iter()
                    .zip(&c_shape)
                    .filter(|&(b, c)| b == c)
                    .count()
                    == c_shape.len()))
        {
            println!("Both dimension size and shape for Add operator should be equal!");
            return Status::InitFailed;
        }

        println!("TVM Add operator init success!");
        Status::Succeed
    }

    fn launch(&self, mut inputs: Vec<Tensor>, output: Tensor) -> (Status, Tensor) {
        if inputs.len() != 2 {
            println!("Inputs tensor length should be 2!");
            return (Status::LaunchFailed, Tensor::default());
        }
        let mut l_tensor = inputs.get_mut(0).unwrap().as_dltensor();
        let mut r_tensor = inputs.get_mut(1).unwrap().as_dltensor();
        let mut out_tensor = output.as_dltensor();

        unsafe {
            // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
            // API calls.
            __wasm_call_ctors();
        }
        let syslib = SystemLibModule::default();
        let add = syslib.get_function("add").expect("add function not found!");
        call_packed!(add, &mut l_tensor, &mut r_tensor, &mut out_tensor).unwrap();

        let output: Tensor = out_tensor.into();
        println!("TVM Add operator run success!");
        (Status::Succeed, output)
    }
}
