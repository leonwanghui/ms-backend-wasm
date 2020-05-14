use anyhow::Result;
use wasmtime::*;

fn main() -> Result<()> {
    let store = Store::default();
    let module = Module::from_file(&store, "/opt/ms_backend_wasm.wasm")?;
    let instance = Instance::new(&module, &[])?;

    // Invoke `add` export
    let add = instance
        .get_func("add")
        .ok_or(anyhow::format_err!("failed to find `add` function export"))?
        .get2::<f32, f32, f32>()?;

    println!("add(6.1, 27.2) = {}", add(6.1, 27.2)?);
    Ok(())
}
