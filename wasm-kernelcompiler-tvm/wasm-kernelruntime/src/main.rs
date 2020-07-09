#[macro_use]
extern crate serde_derive;

pub mod types;
use types::*;

use anyhow::Result;
use getopts::Options;
use serde_json;
// use serde_json::Value;
use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::Read;
use ndarray::Array;
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
        "set the operator type, currently ONLY support Add and Sub, default: Add.",
        "VALUE",
    );

    // opts.optopt("I", "input", "set the input data", "VALUE");
    // opts.optopt("i", "input-data-file", "set input data file", "FILE_PATH");
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
    // let input_data_str: String = match matches.opt_str("I") {
    //     Some(s) => s,
    //     None => {
    //         let input_data_file: String = match matches.opt_str("i") {
    //             Some(s) => s,
    //             None => String::from("/opt/ms-backend-wasm/inputs.json"),
    //         };
    //         let file = File::open(input_data_file).expect("Unable to open file");
    //         let mut buf_reader = BufReader::new(file);
    //         let mut contents = String::new();
    //         buf_reader
    //             .read_to_string(&mut contents)
    //             .expect("Unable to read file");
    //         contents
    //     }
    // };
    // let input_data_val: Value = serde_json::from_str(&input_data_str).unwrap();
    // let input_data = utils::value_to_vec_tensor_input(input_data_val, data_type);
    let a = Array::from_vec(vec![1f32, 2., 3., 4.]);
    let b = Array::from_vec(vec![1f32; 4]);
    let c = Array::from_vec(vec![0f32; 4]);
    let a_tensor: Tensor = a.into();
    let b_tensor: Tensor = b.into();
    let c_tensor: Tensor = c.into();

    let result: Tensor = match execute(
        wasm_backend_file,
        op_type,
        vec![a_tensor, b_tensor, c_tensor],
    ) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&result.to_vec::<f32>()).unwrap()
    );
}

fn execute(wasm_backend_file: String, op_type: i32, input_data: Vec<Tensor>) -> Result<Tensor> {
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

    // Specify the wasm address to access the wasm memory.
    let wasm_addr = memory.data_size();
    // Serialize the data into a JSON string.
    let in_data = serde_json::to_vec(&input_data)?;
    let in_size = in_data.len();
    // Grow up memory size according to in_size to avoid memory leak.
    memory.grow((in_size >> 16) as u32 + 1)?;

    // Insert the input data into wasm memory.
    for i in 0..in_size {
        unsafe {
            memory.data_unchecked_mut()[wasm_addr + i] = *in_data.get(i).unwrap();
        }
    }

    // Invoke `run` export
    let run = instance
        .get_func("run")
        .ok_or(anyhow::format_err!("failed to find `run` function export!"))?
        .get3::<i32, i32, i32, i32>()?;

    let out_size = run(op_type.clone() as i32, wasm_addr as i32, in_size as i32)?;
    if out_size == 0 {
        panic!("Opeartor {:?} run failed!", op_type);
    }

    let out_data = unsafe { &memory.data_unchecked()[wasm_addr..][..out_size as usize] };
    let out_vec: Tensor = serde_json::from_slice(out_data).unwrap();
    Ok(out_vec.clone())
}
