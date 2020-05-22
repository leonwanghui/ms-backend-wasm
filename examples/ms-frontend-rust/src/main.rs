#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NumberType {
    FP32(f32),
    INT8(i8),
    DEFAULT(usize),
}

impl From<f32> for NumberType {
    fn from(num: f32) -> Self {
        NumberType::FP32(num)
    }
}

impl From<i8> for NumberType {
    fn from(num: i8) -> Self {
        NumberType::INT8(num)
    }
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

    // Choose op_type to change the operator type with the follow options:
    //   0: AddOp, 1: MulOp, 2: ArgmaxOp, 3: EqualCountOp, 4..: AddOp
    let op_type: i32 = 0;
    // Choose num_type to change the element type with the follow options:
    //   0: f32, 1: i8, 2..: f32
    let num_type: i32 = 0;
    let in_addr = 0x1000;
    let out_addr = 0x2000;

    let mut input_vec = Vec::new();
    input_vec.push(Box::new(vec![
        NumberType::from(1.0f32),
        NumberType::from(2.0f32),
        NumberType::from(3.0f32),
    ]));
    input_vec.push(Box::new(vec![
        NumberType::from(4.0f32),
        NumberType::from(5.0f32),
        NumberType::from(6.0f32),
    ]));
    println!("input = {:?}", input_vec);
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
        .get5::<i32, i32, i32, i32, i32, i32>()?;

    let out_size = run(
        op_type,
        num_type,
        in_addr as i32,
        in_size as i32,
        out_addr as i32,
    )?;
    if out_size == 0 {
        panic!("Opeartor {} run failed!", op_type);
    }

    let out_data = unsafe { &memory.data_unchecked()[out_addr..][..out_size as usize] };
    let out_vec: Vec<Box<Vec<NumberType>>> = serde_json::from_slice(out_data).unwrap();
    println!("output = {:?}", out_vec);
    Ok(())
}
