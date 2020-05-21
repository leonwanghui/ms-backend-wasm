use anyhow::Result;
use serde_json;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

pub struct Address<T> {
    pub addr: *const T,
    pub size: i32,
}

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

    let op_type: i32 = 0;
    let in_addr = 0x1000;
    let out_addr = 0x2000;

    let mut input_vec = Vec::new();
    input_vec.push(Box::new(vec![1, 2, 3]));
    input_vec.push(Box::new(vec![4, 5, 6]));
    println!("input = {:?}", input_vec);
    let mut out_vec = Vec::new();
    out_vec.push(Box::new(vec![0, 0, 0]));
    // Serialize the data into a JSON string.
    let in_data = serde_json::to_vec(&input_vec)?;
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
        .get4::<i32, i32, i32, i32, i32>()?;

    let out_size = run(op_type, in_addr as i32, in_size as i32, out_addr as i32)?;

    let out_data = unsafe { &memory.data_unchecked()[out_addr..][..out_size as usize] };
    let out_vec: Vec<Box<Vec<i32>>> = serde_json::from_slice(out_data).unwrap();
    println!("output = {:?}", out_vec);
    Ok(())
}
