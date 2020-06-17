use std::{
    convert::From,
    mem,
    os::raw::{c_int, c_void},
    slice,
};
pub use tvm_common::ffi::DLTensor;
use tvm_common::ffi::{
    DLContext, DLDataType, DLDataTypeCode_kDLFloat, DLDataTypeCode_kDLInt, DLDeviceType_kDLCPU,
};

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

impl DataType {
    pub fn as_dldtype(&self) -> DLDataType {
        match self {
            DataType::INT32 => DLDataType {
                bits: DLDataTypeCode_kDLInt as u8,
                code: 32u8,
                lanes: 1u16,
            },
            DataType::INT8 => DLDataType {
                bits: DLDataTypeCode_kDLInt as u8,
                code: 8u8,
                lanes: 1u16,
            },
            DataType::FP32 => DLDataType {
                bits: DLDataTypeCode_kDLFloat as u8,
                code: 32u8,
                lanes: 1u16,
            },
        }
    }
}

impl From<DLDataType> for DataType {
    fn from(dl_dtype: DLDataType) -> Self {
        if dl_dtype.code == DLDataTypeCode_kDLInt as u8 && dl_dtype.bits == 32u8 {
            DataType::INT32
        } else if dl_dtype.code == DLDataTypeCode_kDLInt as u8 && dl_dtype.bits == 8u8 {
            DataType::INT8
        } else if dl_dtype.code == DLDataTypeCode_kDLFloat as u8 && dl_dtype.bits == 32u8 {
            DataType::FP32
        } else {
            DataType::FP32
        }
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
        let data = unsafe {
            match self.dtype() {
                DataType::INT32 => slice::from_raw_parts_mut(
                    self.data().as_mut_ptr() as *mut i32,
                    self.data().len() / mem::size_of::<i32>(),
                )
                .as_mut_ptr() as *mut c_void,
                DataType::INT8 => slice::from_raw_parts_mut(
                    self.data().as_mut_ptr() as *mut i8,
                    self.data().len() / mem::size_of::<i8>(),
                )
                .as_mut_ptr() as *mut c_void,
                DataType::FP32 => slice::from_raw_parts_mut(
                    self.data().as_mut_ptr() as *mut f32,
                    self.data().len() / mem::size_of::<f32>(),
                )
                .as_mut_ptr() as *mut c_void,
            }
        };

        DLTensor {
            data,
            ctx: DLContext {
                device_type: DLDeviceType_kDLCPU,
                device_id: 0 as c_int,
            },
            ndim: self.shape.len() as c_int,
            dtype: self.dtype().as_dldtype(),
            shape: self.shape().as_ptr() as *mut i64,
            strides: self.strides.as_ref().unwrap().as_ptr() as *mut i64,
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
