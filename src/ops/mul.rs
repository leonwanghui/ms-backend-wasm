use super::types::*;
use ndarray::Array;
use std::boxed::Box;

pub struct MulOp {
    data_type: Option<DataType>,
    a_shape: Option<(usize, usize, usize)>,
    a_dim_size: usize,
    b_shape: Option<(usize, usize, usize)>,
    b_dim_size: usize,
}

impl MulOp {
    pub fn new() -> MulOp {
        MulOp {
            data_type: None,
            a_shape: None,
            a_dim_size: 0,
            b_shape: None,
            b_dim_size: 0,
        }
    }

    fn inner_run_fp32(&self, left_vec: Vec<f32>, right_vec: Vec<f32>) -> TensorResult {
        match self.b_dim_size {
            0 => {
                let result = left_vec[0] * right_vec[0];
                TensorResult::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = right.dot(&left);
                TensorResult::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            2 => {
                let left = Array::from_shape_vec(
                    (self.a_shape.unwrap().0, self.a_shape.unwrap().1),
                    left_vec.clone(),
                )
                .unwrap();
                let right = Array::from_shape_vec(
                    (self.b_shape.unwrap().0, self.b_shape.unwrap().1),
                    right_vec.clone(),
                )
                .unwrap();
                let result = right.dot(&left);
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            _ => TensorResult::default(),
        }
    }

    fn inner_run_int32(&self, left_vec: Vec<i32>, right_vec: Vec<i32>) -> TensorResult {
        match self.b_dim_size {
            0 => {
                let result = left_vec[0] * right_vec[0];
                TensorResult::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = right.dot(&left);
                TensorResult::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            2 => {
                let left = Array::from_shape_vec(
                    (self.a_shape.unwrap().0, self.a_shape.unwrap().1),
                    left_vec.clone(),
                )
                .unwrap();
                let right = Array::from_shape_vec(
                    (self.b_shape.unwrap().0, self.b_shape.unwrap().1),
                    right_vec.clone(),
                )
                .unwrap();
                let result = right.dot(&left);
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            _ => TensorResult::default(),
        }
    }

    fn inner_run_int8(&self, left_vec: Vec<i8>, right_vec: Vec<i8>) -> TensorResult {
        match self.b_dim_size {
            0 => {
                let result = left_vec[0] * right_vec[0];
                TensorResult::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = right.dot(&left);
                TensorResult::new(Tensor::from(vec![result]), &Vec::new(), 0)
            }
            2 => {
                let left = Array::from_shape_vec(
                    (self.a_shape.unwrap().0, self.a_shape.unwrap().1),
                    left_vec.clone(),
                )
                .unwrap();
                let right = Array::from_shape_vec(
                    (self.b_shape.unwrap().0, self.b_shape.unwrap().1),
                    right_vec.clone(),
                )
                .unwrap();
                let result = right.dot(&left);
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    result.shape(),
                    result.ndim(),
                )
            }
            _ => TensorResult::default(),
        }
    }
}

impl Operator for MulOp {
    fn init(
        &mut self,
        data_type: DataType,
        a_shape: (usize, usize, usize),
        a_dim_size: usize,
        b_shape: (usize, usize, usize),
        b_dim_size: usize,
    ) -> Status {
        if b_dim_size == 1 {
            if a_shape.0 != b_shape.0 {
                println!("Inputs of 1D array must be same length for Mul operator!");
                return Status::InitFailed;
            }
        } else if b_dim_size == 2 {
            if a_shape.1 != b_shape.0 {
                println!("Inputs of 2D array must be the shape of MxN and NxK for Mul operator!");
                return Status::InitFailed;
            }
        }

        self.data_type = Some(data_type);
        self.a_shape = Some(a_shape);
        self.a_dim_size = a_dim_size;
        self.b_shape = Some(b_shape);
        self.b_dim_size = b_dim_size;
        println!("Mul operator init success!");
        Status::Succeed
    }

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (Status, Vec<Box<TensorResult>>) {
        if inputs.len() != 2 {
            println!("Inputs vector length should be 2!");
            return (
                Status::LaunchFailed,
                vec![Box::new(TensorResult::default())],
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
            _ => TensorResult::default(),
        };
        output_vec.push(Box::new(result));

        println!("Mul operator run success!");
        (Status::Succeed, output_vec)
    }
}
