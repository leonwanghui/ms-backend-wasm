#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Add = 0,
    Mul,
    Argmax,
    EqualCount,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum DataType {
    FP32 = 0,
    INT8,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Tensor {
    Boolean(bool),
    Numeric(usize),
    FP32Array(Vec<f32>),
    INT8Array(Vec<i8>),
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

impl From<Vec<i8>> for Tensor {
    fn from(data: Vec<i8>) -> Self {
        Tensor::INT8Array(data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorResult {
    pub data: Option<Tensor>,
    shape: Option<(usize, usize, usize)>,
    dim_size: usize,
}
