use super::types::*;
use ndarray::Array;
use std::boxed::Box;

pub struct AddOp {
    data_type: Option<DataType>,
    shape: Vec<usize>,
    dim_size: usize,
}

impl AddOp {
    pub fn new() -> AddOp {
        AddOp {
            data_type: None,
            shape: Vec::new(),
            dim_size: 0,
        }
    }

    fn inner_run_fp32(&self, left_vec: Vec<f32>, right_vec: Vec<f32>) -> TensorWrapper {
        match self.dim_size {
            0 => {
                let result = left_vec[0] + right_vec[0];
                TensorWrapper::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = &left + &right;
                TensorWrapper::new(Tensor::from(result.to_vec()), result.shape(), result.ndim())
            }
            2 => {
                let left = Array::from_shape_vec((self.shape[0], self.shape[1]), left_vec.clone())
                    .unwrap();
                let right =
                    Array::from_shape_vec((self.shape[0], self.shape[1]), right_vec.clone())
                        .unwrap();
                let result = &left + &right;
                TensorWrapper::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            3 => {
                let left = Array::from_shape_vec(
                    (self.shape[0], self.shape[1], self.shape[2]),
                    left_vec.clone(),
                )
                .unwrap();
                let right = Array::from_shape_vec(
                    (self.shape[0], self.shape[1], self.shape[2]),
                    right_vec.clone(),
                )
                .unwrap();
                let result = &left + &right;
                TensorWrapper::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            _ => TensorWrapper::default(),
        }
    }

    fn inner_run_int32(&self, left_vec: Vec<i32>, right_vec: Vec<i32>) -> TensorWrapper {
        match self.dim_size {
            0 => {
                let result = left_vec[0] + right_vec[0];
                TensorWrapper::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = &left + &right;
                TensorWrapper::new(Tensor::from(result.to_vec()), result.shape(), result.ndim())
            }
            2 => {
                let left = Array::from_shape_vec((self.shape[0], self.shape[1]), left_vec.clone())
                    .unwrap();
                let right =
                    Array::from_shape_vec((self.shape[0], self.shape[1]), right_vec.clone())
                        .unwrap();
                let result = &left + &right;
                TensorWrapper::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            3 => {
                let left = Array::from_shape_vec(
                    (self.shape[0], self.shape[1], self.shape[2]),
                    left_vec.clone(),
                )
                .unwrap();
                let right = Array::from_shape_vec(
                    (self.shape[0], self.shape[1], self.shape[2]),
                    right_vec.clone(),
                )
                .unwrap();
                let result = &left + &right;
                TensorWrapper::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            _ => TensorWrapper::default(),
        }
    }

    fn inner_run_int8(&self, left_vec: Vec<i8>, right_vec: Vec<i8>) -> TensorWrapper {
        match self.dim_size {
            0 => {
                let result = left_vec[0] + right_vec[0];
                TensorWrapper::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = &left + &right;
                TensorWrapper::new(Tensor::from(result.to_vec()), result.shape(), result.ndim())
            }
            2 => {
                let left = Array::from_shape_vec((self.shape[0], self.shape[1]), left_vec.clone())
                    .unwrap();
                let right =
                    Array::from_shape_vec((self.shape[0], self.shape[1]), right_vec.clone())
                        .unwrap();
                let result = &left + &right;
                TensorWrapper::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            3 => {
                let left = Array::from_shape_vec(
                    (self.shape[0], self.shape[1], self.shape[2]),
                    left_vec.clone(),
                )
                .unwrap();
                let right = Array::from_shape_vec(
                    (self.shape[0], self.shape[1], self.shape[2]),
                    right_vec.clone(),
                )
                .unwrap();
                let result = &left + &right;
                TensorWrapper::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            _ => TensorWrapper::default(),
        }
    }
}

impl Operator for AddOp {
    fn init(
        &mut self,
        data_type: DataType,
        a_shape: Vec<usize>,
        a_dim_size: usize,
        b_shape: Vec<usize>,
        b_dim_size: usize,
    ) -> Status {
        if a_dim_size != b_dim_size || a_shape.len() != b_shape.len() {
            println!("Both dimension size and shape for Add operator should be equal!");
            return Status::InitFailed;
        }

        self.data_type = Some(data_type);
        self.shape = a_shape;
        self.dim_size = a_dim_size;
        println!("Add operator init success!");
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

        let mut output_vec = Vec::new();
        let result = match self.data_type {
            Some(DataType::FP32) => {
                let left_vec = inputs[0].cast_fp32_array();
                let right_vec = inputs[1].cast_fp32_array();
                self.inner_run_fp32(left_vec, right_vec)
            }
            Some(DataType::INT32) => {
                let left_vec = inputs[0].cast_int32_array();
                let right_vec = inputs[1].cast_int32_array();
                self.inner_run_int32(left_vec, right_vec)
            }
            Some(DataType::INT8) => {
                let left_vec = inputs[0].cast_int8_array();
                let right_vec = inputs[1].cast_int8_array();
                self.inner_run_int8(left_vec, right_vec)
            }
            _ => TensorWrapper::default(),
        };
        output_vec.push(Box::new(result));

        println!("Add operator run success!");
        (Status::Succeed, output_vec)
    }
}
