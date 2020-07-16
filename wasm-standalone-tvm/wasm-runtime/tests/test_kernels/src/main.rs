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

use getopts::Options;
use ndarray::Array;
use serde_json;
use std::env;
use wasm_runtime::{KernelExecutor, Tensor};

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "c",
        "ms-backend-config",
        "set wasm backend config file",
        "FILE_PATH",
    );
    opts.optopt(
        "o",
        "op-type",
        "set the operator type, currently ONLY support Add and Sub, default: Add.",
        "VALUE",
    );

    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let wasm_backend_file: String = match matches.opt_str("c") {
        Some(s) => s,
        None => String::from(""),
    };
    let op_type_str: String = match matches.opt_str("o") {
        Some(s) => s,
        None => String::from(""),
    };
    let op_type: i32 = match op_type_str.as_str() {
        "Add" => 0,
        "Sub" => 1,
        _ => 0,
    };

    let a = Array::from_vec(vec![1f32, 2., 3., 4.]);
    let b = Array::from_vec(vec![1f32; 4]);
    let c = Array::from_vec(vec![0f32; 4]);
    let a_tensor: Tensor = a.into();
    let b_tensor: Tensor = b.into();
    let c_tensor: Tensor = c.into();

    let kernel_exec = KernelExecutor::new();
    kernel_exec.instantiate(wasm_backend_file).unwrap();
    kernel_exec
        .set_input(vec![a_tensor, b_tensor, c_tensor])
        .unwrap();
    kernel_exec.run(op_type).unwrap();
    let output: Tensor = match kernel_exec.get_output() {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&result.to_vec::<f32>()).unwrap()
    );
}
