use std::{
    convert::From,
    os::raw::{c_int, c_void},
    slice,
};
pub use tvm_common::ffi::DLTensor;
use tvm_common::ffi::{DLContext, DLDataType};

pub trait Operator {
    fn init(&mut self, dtype: DataType, a_shape: Vec<usize>, b_shape: Vec<usize>) -> Status;

    fn launch(&self, inputs: Vec<*mut DLTensor>, output: *mut DLTensor) -> Status;
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Succeed,
    ParseFailed,
    InitFailed,
    LaunchFailed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Add,
}

impl From<i32> for OpType {
    fn from(op_type: i32) -> Self {
        match op_type {
            0i32 => OpType::Add,
            _ => OpType::Add,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum DataType {
    FP32,
    INT32,
    INT8,
}

impl From<DataType> for DLDataType {
    fn from(dtype: DataType) -> Self {
        let tvm_dtype = match dtype {
            DataType::INT32 => DLDataType {
                bits: 0u8,
                code: 32u8,
                lanes: 1u16,
            },
            DataType::INT8 => DLDataType {
                bits: 0u8,
                code: 8u8,
                lanes: 1u16,
            },
            DataType::FP32 => DLDataType {
                bits: 2u8,
                code: 32u8,
                lanes: 1u16,
            },
        };

        tvm_dtype
    }
}

impl From<DLDataType> for DataType {
    fn from(dl_dtype: DLDataType) -> Self {
        let dtype: DataType = if dl_dtype.code == 0u8 && dl_dtype.bits == 32u8 {
            DataType::INT32
        } else if dl_dtype.code == 0u8 && dl_dtype.bits == 8u8 {
            DataType::INT8
        } else if dl_dtype.code == 2u8 && dl_dtype.bits == 32u8 {
            DataType::FP32
        } else {
            DataType::FP32
        };

        dtype
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tensor {
    pub(crate) dtype: DataType,
    pub(crate) shape: Vec<usize>,
    pub(crate) strides: Option<Vec<usize>>,
    pub(crate) data: Vec<u8>,
}

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

    pub fn as_dltensor(&self) -> DLTensor {
        DLTensor {
            data: unsafe { self.data().as_mut_ptr().offset(self.data.len() as isize) }
                as *mut c_void,
            ctx: DLContext::default(),
            ndim: self.shape.len() as c_int,
            dtype: DLDataType::from(self.dtype()),
            shape: self.shape().as_ptr() as *mut i64,
            strides: self.strides.as_ref().unwrap().as_ptr() as *mut i64,
            byte_offset: self.data.len() as u64,
            ..Default::default()
        }
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

impl From<DLTensor> for Tensor {
    fn from(dlt: DLTensor) -> Self {
        unsafe {
            Self {
                dtype: DataType::from(dlt.dtype),
                shape: slice::from_raw_parts_mut(dlt.shape as *mut usize, dlt.ndim as usize)
                    .to_vec(),
                strides: if dlt.strides.is_null() {
                    None
                } else {
                    Some(
                        slice::from_raw_parts_mut(dlt.strides as *mut usize, dlt.ndim as usize)
                            .to_vec(),
                    )
                },
                data: slice::from_raw_parts_mut(dlt.strides as *mut u8, dlt.byte_offset as usize)
                    .to_vec(),
            }
        }
    }
}
