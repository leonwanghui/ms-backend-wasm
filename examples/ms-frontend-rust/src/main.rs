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

    let op_type: i32 = 0;
    let in_l_addr = 0x1000;
    let in_r_addr = 0x2000;
    let out_addr = 0x3000;
    let in_size: i32 = 3;
    let out_size: i32 = 3;
    for i in 1..4 {
        unsafe {
            memory.data_unchecked_mut()[in_l_addr + i] = i as u8;
            memory.data_unchecked_mut()[in_r_addr + i] = i as u8;
        }
    }

    // Invoke `run` export
    let run = instance
        .get_func("run")
        .ok_or(anyhow::format_err!("failed to find `run` function export!"))?
        .get7::<i32, i32, i32, i32, i32, i32, i32, i32>()?;

    println!(
        "run({}) = {}",
        op_type,
        run(
            op_type,
            in_l_addr as i32,
            in_size,
            in_r_addr as i32,
            in_size,
            out_addr as i32,
            out_size
        )?
    );
    unsafe {
        assert_eq!(memory.data_unchecked()[0x3000], 2);
        assert_eq!(memory.data_unchecked()[0x3001], 4);
        assert_eq!(memory.data_unchecked()[0x3002], 6);
    }
    Ok(())
}
