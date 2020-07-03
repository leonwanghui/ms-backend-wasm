#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
extern crate ndarray;
extern crate tvm_runtime;

mod types;
use types::Tensor;
mod utils;

use std::{collections::HashMap, convert::TryFrom, env, sync::Mutex};
use tvm_runtime::{Graph, GraphExecutor, SystemLibModule, Tensor as TVMTensor};

extern "C" {
    fn __wasm_call_ctors();
}

lazy_static! {
    static ref SYSLIB: SystemLibModule = SystemLibModule::default();
    static ref GRAPH_EXECUTOR: Mutex<GraphExecutor<'static, 'static>> = {
        unsafe {
            // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
            // API calls.
            __wasm_call_ctors();
        }
        let graph = Graph::try_from(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/lib/graph.json"
        )))
        .unwrap();
        let params_bytes =
            include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/lib/graph.params"));
        let params = tvm_runtime::load_param_dict(params_bytes)
            .unwrap()
            .into_iter()
            .map(|(k, v)| (k, v.to_owned()))
            .collect::<HashMap<String, TVMTensor<'static>>>();

        let mut exec = GraphExecutor::new(graph, &*SYSLIB).unwrap();
        exec.load_params(params);

        Mutex::new(exec)
    };
}

#[no_mangle]
pub extern "C" fn run(in_addr: i32, in_size: i32, out_addr: i32) -> i32 {
    println!("in_addr = {:x?}, in_size = {}", in_addr, in_size);
    let in_tensor = utils::load_input(in_addr, in_size as usize);
    let input: TVMTensor = in_tensor.as_dltensor().into();

    GRAPH_EXECUTOR.lock().unwrap().set_input("data", input);
    GRAPH_EXECUTOR.lock().unwrap().run();
    let output = GRAPH_EXECUTOR
        .lock()
        .unwrap()
        .get_output(0)
        .unwrap()
        .as_dltensor(false);

    let out_tensor: Tensor = output.into();
    let out_size = utils::store_output(out_addr, out_tensor);
    out_size as i32
}
