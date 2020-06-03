#[macro_use]
extern crate serde_derive;

pub mod types;
use types::*;
mod utils;

use anyhow::Result;
use getopts::Options;
use serde_json;
use serde_json::Value;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

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
        "set the operator type, ONLY supports Add, Mul, Argmax and EqualCount, default: Add.",
        "VALUE",
    );
    opts.optopt(
        "d",
        "data-type",
        "set the data type, ONLY supports FP32, INT32 and INT8, default: FP32.",
        "VALUE",
    );
    opts.optopt("I", "input", "set the input data", "VALUE");
    opts.optopt("i", "input-data-file", "set input data file", "FILE_PATH");
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
        None => String::from("/opt/ms-backend-wasm/ms_backend_wasm.wasi.wasm"),
    };
    let op_type_str: String = match matches.opt_str("o") {
        Some(s) => s,
        None => String::from("Add"),
    };
    let op_type: i32 = match op_type_str.as_str() {
        "Add" => 0,
        "Mul" => 1,
        "Argmax" => 2,
        "EqualCount" => 3,
        _ => 0,
    };
    let data_type_str: String = match matches.opt_str("d") {
        Some(s) => s,
        None => String::from("FP32"),
    };
    let data_type: i32 = match data_type_str.as_str() {
        "FP32" => 0,
        "INT32" => 1,
        "INT8" => 2,
        _ => 0,
    };
    let input_data_str: String = match matches.opt_str("I") {
        Some(s) => s,
        None => {
            let input_data_file: String = match matches.opt_str("i") {
                Some(s) => s,
                None => String::from("/opt/ms-backend-wasm/inputs.json"),
            };
            let file = File::open(input_data_file).expect("Unable to open file");
            let mut buf_reader = BufReader::new(file);
            let mut contents = String::new();
            buf_reader
                .read_to_string(&mut contents)
                .expect("Unable to read file");
            contents
        }
    };
    let input_data_val: Value = serde_json::from_str(&input_data_str).unwrap();
    let input_data = utils::value_to_vec_tensor_input(input_data_val, data_type);

    let tensor: TensorWrapper = match execute(wasm_backend_file, op_type, data_type, input_data) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    println!("{}", serde_json::to_string_pretty(&tensor).unwrap());
}

fn execute(
    wasm_backend_file: String,
    op_type: i32,
    data_type: i32,
    input_data: Vec<TensorWrapper>,
) -> Result<TensorWrapper> {
    let store = Store::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&store);
    // Create an instance of `Wasi` which contains a `WasiCtx`. Note that
    // `WasiCtx` provides a number of ways to configure what the target program
    // will have access to.
    let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
    wasi.add_to_linker(&mut linker)?;

    let module = Module::from_file(store.engine(), &wasm_backend_file)?;
    let instance = linker.instantiate(&module)?;
    let memory = instance
        .get_memory("memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    // Specify the input address and output address to access the wasm memory.
    let in_addr = 0x1000;
    let out_addr = 0x2000;

    // Serialize the data into a JSON string.
    let in_data = serde_json::to_vec(&input_data)?;
    let in_size = in_data.len();
    // Insert the input data into wasm memory.
    for i in 0..in_size {
        unsafe {
            memory.data_unchecked_mut()[in_addr + i] = *in_data.get(i).unwrap();
        }
    }

    // Invoke `run` export
    let run = instance
        .get_func("run")
        .ok_or(anyhow::format_err!("failed to find `run` function export!"))?
        .get5::<i32, i32, i32, i32, i32, i32>()?;

    let out_size = run(
        op_type.clone() as i32,
        data_type.clone() as i32,
        in_addr as i32,
        in_size as i32,
        out_addr as i32,
    )?;
    if out_size == 0 {
        panic!("Opeartor {:?} run failed!", op_type);
    }

    let out_data = unsafe { &memory.data_unchecked()[out_addr..][..out_size as usize] };
    let out_vec: Vec<TensorWrapper> = serde_json::from_slice(out_data).unwrap();
    Ok(out_vec[0].clone())
}
