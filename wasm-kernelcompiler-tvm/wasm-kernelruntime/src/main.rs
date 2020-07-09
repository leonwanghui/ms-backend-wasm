#[macro_use]
extern crate serde_derive;

pub mod types;
use types::Tensor;
mod runtime;

use getopts::Options;
use serde_json;
// use serde_json::Value;
use std::env;
// use std::fs::File;
// use std::io::BufReader;
// use std::io::Read;
use ndarray::Array;

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

    let result: Tensor = match runtime::execute(
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
