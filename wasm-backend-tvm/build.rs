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

use std::{path::PathBuf, process::Command};

fn main() {
    let mut out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    out_dir.push("lib");

    if !out_dir.is_dir() {
        std::fs::create_dir(&out_dir).unwrap();
    }

    let obj_file = out_dir.join("add.o");
    let lib_file = out_dir.join("libops_wasm32.a");

    Command::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tools/build_ops_lib.py"
    ))
    .arg(&out_dir)
    .output()
    .expect("Failed to execute command");

    let ar = option_env!("LLVM_AR").unwrap_or("llvm-ar-8");
    Command::new(ar)
        .arg("rcs")
        .arg(&lib_file)
        .arg(&obj_file)
        .output()
        .expect("Failed to execute command");

    println!("cargo:rustc-link-lib=static=ops_wasm32");
    println!("cargo:rustc-link-search=native={}", out_dir.display());
}
