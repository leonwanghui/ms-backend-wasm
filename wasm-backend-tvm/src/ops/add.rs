use super::types::*;
use tvm_runtime::{Module as _, SystemLibModule};

extern "C" {
    static __tvm_module_ctx: i32;
    fn __wasm_call_ctors();
}

#[no_mangle]
unsafe fn __get_tvm_module_ctx() -> i32 {
    // Refer a symbol in the libops_wasm32.a to make sure that the link of the
    // library is not optimized out.
    __tvm_module_ctx
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

    fn launch(&self, inputs: Vec<*mut DLTensor>, output: *mut DLTensor) -> Status {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return Status::LaunchFailed;
        }

        unsafe {
            // This is necessary to invoke TVMBackendRegisterSystemLibSymbol
            // API calls.
            __wasm_call_ctors();
        }
        let syslib = SystemLibModule::default();
        let add = syslib.get_function("add").expect("add function not found!");
        call_packed!(add, inputs[0], inputs[1], output).unwrap();

        println!("TVM Add operator run success!");
        Status::Succeed
    }
}
