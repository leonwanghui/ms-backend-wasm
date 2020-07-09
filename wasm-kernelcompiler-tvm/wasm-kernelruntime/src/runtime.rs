use super::types::Tensor;
use anyhow::Result;
use serde_json;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

pub fn execute(wasm_backend_file: String, op_type: i32, input_data: Vec<Tensor>) -> Result<Tensor> {
    let engine = Engine::new(Config::new().wasm_simd(true));
    let store = Store::new(&engine);

    // First set up our linker which is going to be linking modules together. We
    // want our linker to have wasi available, so we set that up here as well.
    let mut linker = Linker::new(&store);
    // Create an instance of `Wasi` which contains a `WasiCtx`. Note that
    // `WasiCtx` provides a number of ways to configure what the target program
    // will have access to.
    let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
    wasi.add_to_linker(&mut linker)?;

    let module = Module::from_file(&store, &wasm_backend_file)?;
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
