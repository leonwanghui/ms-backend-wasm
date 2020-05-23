#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod types;
use types::*;

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

fn main() -> Result<()> {
    let store = Store::default();

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&store);
    let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
    wasi.add_to_linker(&mut linker)?;

    let module = Module::from_file(&store, "/opt/ms_backend_wasm.wasi.wasm")?;
    let instance = linker.instantiate(&module)?;
    let memory = instance
        .get_memory("memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    // Choose op_type to change the operator type with the follow options:
    //   * OpType::Add
    //   * OpType::Mul
    //   * OpType::Argmax
    //   * OpType::EqualCount
    let op_type = OpType::Mul;
    // Choose data_type to change the data type with the follow options:
    //   * DataType::FP32
    //   * DataType::INT8
    let data_type = DataType::FP32;
    // Specify the input data, dimension size and shape of left array.
    let a_input_data = vec![1.0f32, 2.0f32, 3.0f32];
    let a_dim_size = 2;
    let a_shape = (1, 3, 0);
    // Specify the input data, dimension size and shape of right array.
    let b_input_data = vec![1.0f32, 5.0f32, 6.0f32];
    let b_dim_size = 2;
    let b_shape = (3, 1, 0);
    // Specify the input address and output address to access the wasm memory.
    let in_addr = 0x1000;
    let out_addr = 0x2000;

    let input_data = vec![
        Box::new(Tensor::from(a_input_data)),
        Box::new(Tensor::from(b_input_data)),
    ];
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
        .get13::<i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32>()?;

    let out_size = run(
        op_type.clone() as i32,
        data_type.clone() as i32,
        a_dim_size as i32,
        a_shape.0 as i32,
        a_shape.1 as i32,
        a_shape.2 as i32,
        b_dim_size as i32,
        b_shape.0 as i32,
        b_shape.1 as i32,
        b_shape.2 as i32,
        in_addr as i32,
        in_size as i32,
        out_addr as i32,
    )?;
    if out_size == 0 {
        panic!("Opeartor {:?} run failed!", op_type);
    }

    let out_data = unsafe { &memory.data_unchecked()[out_addr..][..out_size as usize] };
    let out_vec: Vec<Box<TensorResult>> = serde_json::from_slice(out_data).unwrap();
    println!("output = {:?}", out_vec);
    Ok(())
}
