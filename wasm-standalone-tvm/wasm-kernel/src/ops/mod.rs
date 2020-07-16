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

mod add;
use add::TVMAddOp;
mod sub;
use sub::TVMSubOp;
pub mod types;
use types::*;

use std::boxed::Box;

pub fn operator_instantiate(op_type: i32) -> Box<dyn Operator> {
    match OpType::from(op_type) {
        OpType::Add => Box::new(TVMAddOp::new()),
        OpType::Sub => Box::new(TVMSubOp::new()),
    }
}

pub fn validate_inputs(inputs: &Vec<Tensor>) -> Status {
    if (inputs.len() == 3
        && !(inputs[0].dtype() == inputs[1].dtype() && inputs[0].dtype() == inputs[2].dtype()))
        || (inputs.len() == 2 && inputs[0].dtype() != inputs[1].dtype())
    {
        println!("The dtype of inputs and outputs is not equal!");
        Status::ValidateFailed
    } else {
        Status::Succeed
    }
}

pub fn parse_inputs_shape(inputs: &Vec<Tensor>) -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    if inputs.len() == 3 {
        (inputs[0].shape(), inputs[1].shape(), inputs[2].shape())
    } else {
        (inputs[0].shape(), inputs[1].shape(), Vec::new())
    }
}

pub fn parse_inputs_tensor(inputs: &Vec<Tensor>) -> (Vec<Tensor>, Tensor) {
    if inputs.len() == 3 {
        (
            vec![inputs[0].clone(), inputs[1].clone()],
            inputs[2].clone(),
        )
    } else {
        (vec![inputs[0].clone()], inputs[1].clone())
    }
}
