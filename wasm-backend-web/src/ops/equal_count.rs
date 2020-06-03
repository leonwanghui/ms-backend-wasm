use super::types::*;
use std::boxed::Box;

pub struct EqualCountOp {
    data_type: Option<DataType>,
    shape: Vec<usize>,
    dim_size: usize,
}

impl EqualCountOp {
    pub fn new() -> EqualCountOp {
        EqualCountOp {
            data_type: None,
            shape: Vec::new(),
            dim_size: 0,
        }
    }
}

impl Operator for EqualCountOp {
    fn init(
        &mut self,
        data_type: DataType,
        a_shape: Vec<usize>,
        a_dim_size: usize,
        b_shape: Vec<usize>,
        b_dim_size: usize,
    ) -> Status {
        if a_dim_size != b_dim_size || a_shape.len() != b_shape.len() {
            println!("Both dimension and shape for EqualCount operator should be equal!");
            return Status::InitFailed;
        }

        self.data_type = Some(data_type);
        self.shape = a_shape;
        self.dim_size = a_dim_size;
        println!("EqualCount operator init success!");
        Status::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (Status, Vec<Box<TensorWrapper>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (
                Status::LaunchFailed,
                vec![Box::new(TensorWrapper::default())],
            );
        }

        let mut num = 0;
        match self.data_type {
            Some(DataType::FP32) => {
                let left = inputs[0].cast_fp32_array();
                let right = inputs[1].cast_fp32_array();
                if left.len() != right.len() {
                    println!("Inputs size not equal!");
                    return (
                        Status::LaunchFailed,
                        vec![Box::new(TensorWrapper::default())],
                    );
                }
                for i in 0..left.len() {
                    if left[i as usize] == right[i as usize] {
                        num += 1;
                    }
                }
            }
            Some(DataType::INT32) => {
                let left = inputs[0].cast_int32_array();
                let right = inputs[1].cast_int32_array();
                if left.len() != right.len() {
                    println!("Inputs size not equal!");
                    return (
                        Status::LaunchFailed,
                        vec![Box::new(TensorWrapper::default())],
                    );
                }
                for i in 0..left.len() {
                    if left[i as usize] == right[i as usize] {
                        num += 1;
                    }
                }
            }
            Some(DataType::INT8) => {
                let left = inputs[0].cast_int8_array();
                let right = inputs[1].cast_int8_array();
                if left.len() != right.len() {
                    println!("Inputs size not equal!");
                    return (
                        Status::LaunchFailed,
                        vec![Box::new(TensorWrapper::default())],
                    );
                }
                for i in 0..left.len() {
                    if left[i as usize] == right[i as usize] {
                        num += 1;
                    }
                }
            }
            _ => {}
        }

        let mut tensor_res = TensorWrapper::default();
        tensor_res.data = Some(Tensor::from(num as usize));
        let mut output_vec = Vec::new();
        output_vec.push(Box::new(tensor_res));
        println!("EqualCount Operator run success!");
        (Status::Succeed, output_vec)
    }
}
