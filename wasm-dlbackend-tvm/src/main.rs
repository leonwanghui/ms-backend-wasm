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
extern crate csv;
extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate ndarray;
extern crate tvm_runtime;

use image::{FilterType, GenericImageView};
use ndarray::Array;
use std::{collections::HashMap, convert::TryFrom, env, sync::Mutex};
use tvm_runtime::{Graph, GraphExecutor, SystemLibModule, Tensor};

const IMG_HEIGHT: usize = 224;
const IMG_WIDTH: usize = 224;

extern "C" {
    fn __wasm_call_ctors();
}

lazy_static! {
    static ref SYSLIB: SystemLibModule = SystemLibModule::default();
    static ref GRAPH_EXECUTOR: Mutex<GraphExecutor<'static, 'static>> = {
        unsafe {
            // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
            // API calls.
            __wasm_call_ctors();
        }
        let graph = Graph::try_from(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/lib/graph.json"
        )))
        .unwrap();
        let params_bytes =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/graph.params"));
        let params = tvm_runtime::load_param_dict(params_bytes)
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k, v.to_owned()))
            .collect::<HashMap<String, Tensor<'static>>>();

        let mut exec = GraphExecutor::new(graph, &*SYSLIB).unwrap();
        exec.load_params(params);

        Mutex::new(exec)
    };
}

fn preprocess(img: image::DynamicImage) -> Tensor<'static> {
    println!("original image dimensions: {:?}", img.dimensions());
    let img = img
        .resize_exact(IMG_HEIGHT as u32, IMG_WIDTH as u32, FilterType::Nearest)
        .to_rgb();
    println!("resized image dimensions: {:?}", img.dimensions());
    let mut pixels: Vec<f32> = vec![];
    for pixel in img.pixels() {
        let tmp = pixel.data;
        // normalize the RGB channels using mean, std of imagenet1k
        let tmp = [
            (tmp[0] as f32 - 123.0) / 58.395, // R
            (tmp[1] as f32 - 117.0) / 57.12,  // G
            (tmp[2] as f32 - 104.0) / 57.375, // B
        ];
        for e in &tmp {
            pixels.push(*e);
        }
    }

    // (H,W,C) -> (C,H,W)
    let arr = Array::from_shape_vec((IMG_HEIGHT, IMG_WIDTH, 3), pixels).unwrap();
    let arr = arr.permuted_axes([2, 0, 1]);
    let arr = Array::from_iter(arr.into_iter().map(|&v| v));

    return Tensor::from(arr);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let img_file = &args[1];
    let img = image::open(img_file).unwrap();
    let input = preprocess(img);

    GRAPH_EXECUTOR.lock().unwrap().set_input("data", input);
    GRAPH_EXECUTOR.lock().unwrap().run();
    let output = GRAPH_EXECUTOR
        .lock()
        .unwrap()
        .get_output(0)
        .unwrap()
        .to_vec::<f32>();

    // find the maximum entry in the output and its index
    let mut argmax = -1;
    let mut max_prob = 0.;
    for i in 0..output.len() {
        if output[i] > max_prob {
            max_prob = output[i];
            argmax = i as i32;
        }
    }

    // create a hash map of (class id, class name)
    let mut synset: HashMap<i32, String> = HashMap::new();
    let mut rdr = csv::ReaderBuilder::new().from_reader(
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/tools/synset.csv")).as_bytes(),
    );

    for result in rdr.records() {
        let record = result.unwrap();
        let id: i32 = record[0].parse().unwrap();
        let cls = record[1].to_string();
        synset.insert(id, cls);
    }

    println!(
        "input image belongs to the class `{}`",
        synset
            .get(&argmax)
            .expect("cannot find the class id for argmax")
    );
}
