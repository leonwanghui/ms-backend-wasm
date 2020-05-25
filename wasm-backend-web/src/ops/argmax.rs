use super::types::*;
use std::boxed::Box;

pub struct ArgmaxOp {
    data_type: Option<DataType>,
    shape: Option<(usize, usize, usize)>,
    dim_size: usize,
}

impl ArgmaxOp {
    pub fn new() -> ArgmaxOp {
        ArgmaxOp {
            data_type: None,
            shape: None,
            dim_size: 0,
        }
    }

    fn inner_run_fp32(&self, input_vec: Vec<f32>) -> TensorResult {
        let mut tensor_res = TensorResult::default();

        if self.dim_size == 1 {
            let mut index = 0;
            let mut max_arg = input_vec[0];

            for i in 0..input_vec.len() {
                if input_vec[i] > max_arg {
                    max_arg = input_vec[i];
                    index = i;
                }
            }
            tensor_res.data = Some(Tensor::from(vec![max_arg, index as f32]));
        }
        tensor_res
    }

    fn inner_run_int8(&self, input_vec: Vec<i8>) -> TensorResult {
        let mut tensor_res = TensorResult::default();

        if self.dim_size == 1 {
            let mut index = 0;
            let mut max_arg = input_vec[0];

            for i in 0..input_vec.len() {
                if input_vec[i] > max_arg {
                    max_arg = input_vec[i];
                    index = i;
                }
            }
            tensor_res.data = Some(Tensor::from(vec![max_arg, index as i8]));
        }
        tensor_res
    }
}

impl Operator for ArgmaxOp {
    #[allow(unused_variables)]
    fn init(
        &mut self,
        data_type: DataType,
        a_shape: (usize, usize, usize),
        a_dim_size: usize,
        b_shape: (usize, usize, usize),
        b_dim_size: usize,
    ) -> Status {
        self.data_type = Some(data_type);
        self.shape = Some(a_shape);
        self.dim_size = a_dim_size;
        println!("Argmax operator init success!");
        Status::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (Status, Vec<Box<TensorResult>>) {
        if inputs.len() == 0 {
            println!("Inputs vector length should not be zero!");
            return (
                Status::LaunchFailed,
                vec![Box::new(TensorResult::default())],
            );
        }

        let mut output_vec = Vec::new();
        let result = match self.data_type {
            Some(DataType::FP32) => {
                let input_vec = inputs[0].cast_fp32_array();
                self.inner_run_fp32(input_vec)
            }
            Some(DataType::INT8) => {
                let input_vec = inputs[0].cast_int8_array();
                self.inner_run_int8(input_vec)
            }
            _ => TensorResult::default(),
        };
        output_vec.push(Box::new(result));

        println!("Argmax operator run success!");
        (Status::Succeed, output_vec)
    }
}
