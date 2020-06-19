use std::boxed::Box;
use std::convert::From;

pub trait Operator {
    fn init(
        &mut self,
        data_type: DataType,
        a_shape: Vec<usize>,
        a_dim_size: usize,
        b_shape: Vec<usize>,
        b_dim_size: usize,
    ) -> Status;

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (Status, Vec<Box<TensorWrapper>>);
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Succeed = 0,
    ParseFailed,
    InitFailed,
    LaunchFailed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Add = 0,
    Mul,
    Argmax,
    EqualCount,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    FP32 = 0,
    INT32,
    INT8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Tensor {
    Boolean(bool),
    Numeric(usize),
    FP32Array(Vec<f32>),
    INT32Array(Vec<i32>),
    INT8Array(Vec<i8>),
}

impl Tensor {
    pub fn cast_fp32_array(&self) -> Vec<f32> {
        if let Tensor::FP32Array(c) = &*self {
            c.to_vec()
        } else {
            unreachable!()
        }
    }

    pub fn cast_int32_array(&self) -> Vec<i32> {
        if let Tensor::INT32Array(c) = &*self {
            c.to_vec()
        } else {
            unreachable!()
        }
    }

    pub fn cast_int8_array(&self) -> Vec<i8> {
        if let Tensor::INT8Array(c) = &*self {
            c.to_vec()
        } else {
            unreachable!()
        }
    }
}

impl From<bool> for Tensor {
    fn from(data: bool) -> Self {
        Tensor::Boolean(data)
    }
}

impl From<usize> for Tensor {
    fn from(data: usize) -> Self {
        Tensor::Numeric(data)
    }
}

impl From<Vec<f32>> for Tensor {
    fn from(data: Vec<f32>) -> Self {
        Tensor::FP32Array(data)
    }
}

impl From<Vec<i32>> for Tensor {
    fn from(data: Vec<i32>) -> Self {
        Tensor::INT32Array(data)
    }
}

impl From<Vec<i8>> for Tensor {
    fn from(data: Vec<i8>) -> Self {
        Tensor::INT8Array(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorWrapper {
    #[serde(rename = "input-data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Tensor>,
    #[serde(rename = "shape", skip_serializing_if = "Option::is_none")]
    pub shape: Option<Vec<usize>>,
    #[serde(rename = "dim-size", skip_serializing_if = "Option::is_none")]
    pub dim_size: Option<usize>,
}

#[allow(dead_code)]
impl TensorWrapper {
    pub fn new(tensor: Tensor, shape: &[usize], dim_size: usize) -> Self {
        let mut tensor_wrap = TensorWrapper::default();

        tensor_wrap.data = Some(tensor);
        if !(shape.is_empty() || dim_size == 0) {
            tensor_wrap.shape = Some((*shape).to_vec());
            tensor_wrap.dim_size = Some(dim_size);
        }

        tensor_wrap
    }
}

impl Default for TensorWrapper {
    fn default() -> Self {
        TensorWrapper {
            data: None,
            shape: None,
            dim_size: None,
        }
    }
}
