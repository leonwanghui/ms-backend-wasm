use super::types::*;
use tvm_runtime::{Module as _, SystemLibModule};

extern "C" {
    fn __wasm_call_ctors();
}

pub struct TVMAddOp {
    data_type: Option<DataType>,
    shape: Vec<usize>,
    dim_size: usize,
}

impl TVMAddOp {
    pub fn new() -> Self {
        Self {
            data_type: None,
            shape: Vec::new(),
            dim_size: 0,
        }
    }
}

impl Operator for TVMAddOp {
    fn init(&mut self, data_type: DataType, a_shape: Vec<usize>, b_shape: Vec<usize>) -> Status {
        if a_shape.len() != b_shape.len()
            && a_shape
                .iter()
                .zip(&b_shape)
                .filter(|&(a, b)| a == b)
                .count()
                != a_shape.len()
        {
            println!("Both dimension size and shape for Add operator should be equal!");
            return Status::InitFailed;
        }

        self.data_type = Some(data_type);
        self.dim_size = a_shape.len();
        self.shape = a_shape;
        println!("TVM Add operator init success!");
        Status::Succeed
    }

    fn launch(&self, inputs: Vec<DLTensor>, output: &mut DLTensor) -> Status {
        if inputs.len() != 2 {
            println!("Inputs tensor length should be 2!");
            return Status::LaunchFailed;
        }
        let l_tensor = inputs.get(0).unwrap();
        let r_tensor = inputs.get(1).unwrap();

        unsafe {
            // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
            // API calls.
            __wasm_call_ctors();
        }
        let syslib = SystemLibModule::default();
        let add = syslib.get_function("add").expect("add function not found!");
        call_packed!(add, l_tensor, r_tensor, output).unwrap();

        println!("TVM Add operator run success!");
        Status::Succeed
    }
}
