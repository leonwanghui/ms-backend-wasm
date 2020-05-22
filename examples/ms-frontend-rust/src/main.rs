#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::{Wasi, WasiCtx};

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub enum NumberType {
//     FP32(f32),
//     INT8(i8),
//     DEFAULT(usize),
// }

// impl From<f32> for NumberType {
//     fn from(num: f32) -> Self {
//         NumberType::FP32(num)
//     }
// }

// impl From<i8> for NumberType {
//     fn from(num: i8) -> Self {
//         NumberType::INT8(num)
//     }
// }

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Tensor {
    Boolean(bool),
    Numeric(f32),
    OneDArray(Vec<f32>),
    TwoDArray(Vec<Vec<f32>>),
    ThreeDArray(Vec<Vec<Vec<f32>>>),
    Default(usize),
}

impl From<bool> for Tensor {
    fn from(data: bool) -> Self {
        Tensor::Boolean(data)
    }
}

impl From<f32> for Tensor {
    fn from(data: f32) -> Self {
        Tensor::Numeric(data)
    }
}

impl From<Vec<f32>> for Tensor {
    fn from(data: Vec<f32>) -> Self {
        Tensor::OneDArray(data)
    }
}

impl From<Vec<Vec<f32>>> for Tensor {
    fn from(data: Vec<Vec<f32>>) -> Self {
        Tensor::TwoDArray(data)
    }
}

impl From<Vec<Vec<Vec<f32>>>> for Tensor {
    fn from(data: Vec<Vec<Vec<f32>>>) -> Self {
        Tensor::ThreeDArray(data)
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
    // Choose data_type to change the data type with the follow options:
    //   0: bool, 1: f32, 2: Vec<f32>, 3: Vec<Vec<f32>>, 4..: Vec<Vec<Vec<f32>>>
    let data_type: i32 = 2;
    let input_data = vec![
        Box::new(Tensor::from(vec![1.0f32, 2.0f32, 3.0f32])),
        Box::new(Tensor::from(vec![4.0f32, 5.0f32, 6.0f32])),
    ];
    let dim_size: i32 = 1;
    let shape_list = (3, 0, 0);
    let in_addr = 0x1000;
    let out_addr = 0x2000;

    println!("input = {:?}", input_data);
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
        .get9::<i32, i32, i32, i32, i32, i32, i32, i32, i32, i32>()?;

    let out_size = run(
        op_type,
        data_type,
        dim_size,
        shape_list.0 as i32,
        shape_list.1 as i32,
        shape_list.2 as i32,
        in_addr as i32,
        in_size as i32,
        out_addr as i32,
    )?;
    if out_size == 0 {
        panic!("Opeartor {} run failed!", op_type);
    }

    let out_data = unsafe { &memory.data_unchecked()[out_addr..][..out_size as usize] };
    let out_vec: Vec<Box<Tensor>> = serde_json::from_slice(out_data).unwrap();
    println!("output = {:?}", out_vec);
    Ok(())
}
