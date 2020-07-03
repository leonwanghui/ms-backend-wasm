#[macro_use]
extern crate serde_derive;
extern crate csv;
extern crate image;

mod types;
use types::Tensor;

use anyhow::Result;
use getopts::Options;
use image::{FilterType, GenericImageView};
use ndarray::Array;
use serde_json;
use std::{collections::HashMap, env};
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

const IMG_HEIGHT: usize = 224;
const IMG_WIDTH: usize = 224;

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
        "i",
        "input-data-file",
        "set the path to input image file",
        "FILE_PATH",
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
    let wasm_backend_file = matches.opt_str("c").unwrap();
    let input_data_file = matches.opt_str("i").unwrap();
    let img = image::open(input_data_file).unwrap();
    let input = data_preprocess(img);

    let output: Tensor = match execute(wasm_backend_file, input) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    output_assert(output);
}

fn data_preprocess(img: image::DynamicImage) -> Tensor {
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

fn execute(wasm_backend_file: String, input_data: Tensor) -> Result<Tensor> {
    let store = Store::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&store);
    // Create an instance of `Wasi` which contains a `WasiCtx`. Note that
    // `WasiCtx` provides a number of ways to configure what the target program
    // will have access to.
    let wasi = Wasi::new(&store, WasiCtx::new(env::args())?);
    wasi.add_to_linker(&mut linker)?;

    let module = Module::from_file(store.engine(), &wasm_backend_file)?;
    let instance = linker.instantiate(&module)?;
    let memory = instance
        .get_memory("memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    // Specify the input address to access the wasm memory.
    let in_addr = 0x1000;
    let out_addr = 0x2000;
    // Serialize the data into a JSON string.
    let in_data = serde_json::to_vec(&input_data)?;
    let in_size = in_data.len();
    assert!(memory.data_size() > in_size);

    // Insert the input data into wasm memory.
    for i in 0..in_size {
        unsafe {
            memory.data_unchecked_mut()[in_addr + i] = *in_data.get(i).unwrap();
        }
    }

    // Invoke `run` export.
    let run = instance
        .get_func("run")
        .ok_or(anyhow::format_err!("failed to find `run` function export!"))?
        .get3::<i32, i32, i32, i32>()?;

    let out_size = run(in_addr as i32, in_size as i32, out_addr as i32)?;
    if out_size == 0 {
        panic!("graph run failed!");
    }

    println!("test2!");
    let out_data = unsafe { &memory.data_unchecked()[out_addr..][..out_size as usize] };
    let out_vec: Tensor = serde_json::from_slice(out_data).unwrap();
    Ok(out_vec)
}

fn output_assert(out_tensor: Tensor) {
    let output = out_tensor.to_vec::<f32>();

    // Find the maximum entry in the output and its index.
    let mut argmax = -1;
    let mut max_prob = 0.;
    for i in 0..output.len() {
        if output[i] > max_prob {
            max_prob = output[i];
            argmax = i as i32;
        }
    }

    // Create a hash map of (class id, class name)
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
