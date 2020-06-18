use std::{convert::From, mem, slice};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DataType {
    FP32,
    INT32,
    INT8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    pub(crate) dtype: DataType,
    pub(crate) shape: Vec<usize>,
    pub(crate) strides: Option<Vec<usize>>,
    pub(crate) data: Vec<u8>,
}

#[allow(dead_code)]
impl Tensor {
    pub fn new(dtype: DataType, shape: Vec<usize>, strides: Vec<usize>, data: Vec<u8>) -> Self {
        Tensor {
            dtype: dtype,
            shape: shape,
            strides: Some(strides),
            data: data,
        }
    }

    pub fn dtype(&self) -> DataType {
        self.dtype.clone()
    }

    pub fn ndim(&self) -> usize {
        self.shape.len()
    }

    pub fn shape(&self) -> Vec<usize> {
        self.shape.clone()
    }

    pub fn data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl Default for Tensor {
    fn default() -> Self {
        Self {
            dtype: DataType::FP32,
            shape: Vec::new(),
            strides: None,
            data: Vec::new(),
        }
    }
}

/// `From` conversions to `Tensor` for `ndarray::Array`.
/// Takes a reference to the `ndarray` since `Tensor` is not owned.
macro_rules! impl_tensor_from_ndarray {
    ($type:ty, $typecode:expr) => {
        impl<D: ndarray::Dimension> From<ndarray::Array<$type, D>> for Tensor {
            fn from(arr: ndarray::Array<$type, D>) -> Self {
                Tensor {
                    dtype: $typecode,
                    shape: arr.shape().to_vec(),
                    strides: Some(arr.strides().iter().map(|v| *v as usize).collect()),
                    data: unsafe {
                        slice::from_raw_parts(
                            arr.as_ptr() as *const u8,
                            arr.len() * mem::size_of::<$type>(),
                        )
                        .to_vec()
                    },
                }
            }
        }
    };
}

impl_tensor_from_ndarray!(f32, DataType::FP32);
impl_tensor_from_ndarray!(i32, DataType::INT32);
impl_tensor_from_ndarray!(i8, DataType::INT8);
