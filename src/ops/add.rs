use super::types::*;
use ndarray::Array;
use std::boxed::Box;

pub struct AddOp {
    data_type: Option<DataType>,
    shape: Option<(usize, usize, usize)>,
    dim_size: usize,
}

impl AddOp {
    pub fn new() -> AddOp {
        AddOp {
            data_type: None,
            shape: None,
            dim_size: 0,
        }
    }

    fn inner_run_fp32(&self, left_vec: Vec<f32>, right_vec: Vec<f32>) -> TensorResult {
        match self.dim_size {
            0 => {
                let result = left_vec[0] + right_vec[0];
                TensorResult::new(Tensor::from(vec![result]), (0, 0, 0), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = &left + &right;
                TensorResult::new(
                    Tensor::from(result.to_vec()),
                    (result.shape()[0], 0, 0),
                    result.ndim(),
                )
            }
            2 => {
                let left = unsafe {
                    Array::from_shape_vec_unchecked(
                        (self.shape.unwrap().0, self.shape.unwrap().1),
                        left_vec.clone(),
                    )
                };
                let right = unsafe {
                    Array::from_shape_vec_unchecked(
                        (self.shape.unwrap().0, self.shape.unwrap().1),
                        left_vec.clone(),
                    )
                };
                let result = &left + &right;
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    (result.shape()[0], result.shape()[1], 0),
                    result.ndim(),
                )
            }
            3 => {
                let left = unsafe {
                    Array::from_shape_vec_unchecked(
                        (
                            self.shape.unwrap().0,
                            self.shape.unwrap().1,
                            self.shape.unwrap().2,
                        ),
                        left_vec.clone(),
                    )
                };
                let right = unsafe {
                    Array::from_shape_vec_unchecked(
                        (
                            self.shape.unwrap().0,
                            self.shape.unwrap().1,
                            self.shape.unwrap().2,
                        ),
                        left_vec.clone(),
                    )
                };
                let result = &left + &right;
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    (result.shape()[0], result.shape()[1], result.shape()[2]),
                    result.ndim(),
                )
            }
            _ => TensorResult::default(),
        }
    }

    fn inner_run_int8(&self, left_vec: Vec<i8>, right_vec: Vec<i8>) -> TensorResult {
        match self.dim_size {
            0 => {
                let result = left_vec[0] + right_vec[0];
                TensorResult::new(Tensor::from(vec![result]), (0, 0, 0), 0)
            }
            1 => {
                let left = Array::from(left_vec);
                let right = Array::from(right_vec);
                let result = &left + &right;
                TensorResult::new(
                    Tensor::from(result.to_vec()),
                    (result.shape()[0], 0, 0),
                    result.ndim(),
                )
            }
            2 => {
                let left = unsafe {
                    Array::from_shape_vec_unchecked(
                        (self.shape.unwrap().0, self.shape.unwrap().1),
                        left_vec.clone(),
                    )
                };
                let right = unsafe {
                    Array::from_shape_vec_unchecked(
                        (self.shape.unwrap().0, self.shape.unwrap().1),
                        left_vec.clone(),
                    )
                };
                let result = &left + &right;
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    (result.shape()[0], result.shape()[1], 0),
                    result.ndim(),
                )
            }
            3 => {
                let left = unsafe {
                    Array::from_shape_vec_unchecked(
                        (
                            self.shape.unwrap().0,
                            self.shape.unwrap().1,
                            self.shape.unwrap().2,
                        ),
                        left_vec.clone(),
                    )
                };
                let right = unsafe {
                    Array::from_shape_vec_unchecked(
                        (
                            self.shape.unwrap().0,
                            self.shape.unwrap().1,
                            self.shape.unwrap().2,
                        ),
                        left_vec.clone(),
                    )
                };
                let result = &left + &right;
                TensorResult::new(
                    Tensor::from(result.as_slice().unwrap().to_vec()),
                    (result.shape()[0], result.shape()[1], result.shape()[2]),
                    result.ndim(),
                )
            }
            _ => TensorResult::default(),
        }
    }
}

impl Operator for AddOp {
    fn init(
        &mut self,
        data_type: DataType,
        a_shape: (usize, usize, usize),
        a_dim_size: usize,
        b_shape: (usize, usize, usize),
        b_dim_size: usize,
    ) -> Status {
        if a_dim_size != b_dim_size
            || a_shape.0 != b_shape.0
            || a_shape.1 != b_shape.1
            || a_shape.2 != b_shape.2
        {
            println!("Both dimension size and shape for Add operator should be equal!");
            return Status::InitFailed;
        }

        self.data_type = Some(data_type);
        self.shape = Some(a_shape);
        self.dim_size = a_dim_size;
        println!("Add operator init success!");
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
            Some(DataType::INT8) => {
                let left_vec = inputs[0].cast_int8_array();
                let right_vec = inputs[1].cast_int8_array();
                self.inner_run_int8(left_vec, right_vec)
            }
            _ => TensorResult::default(),
        };
        output_vec.push(Box::new(result));

        println!("Add operator run success!");
        (Status::Succeed, output_vec)
    }
}
