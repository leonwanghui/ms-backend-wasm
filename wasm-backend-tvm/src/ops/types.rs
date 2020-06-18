use std::{
    convert::From,
    os::raw::{c_int, c_void},
    slice,
};
pub use tvm_common::ffi::DLTensor;
use tvm_common::ffi::{
    DLContext, DLDataType, DLDataTypeCode_kDLFloat, DLDataTypeCode_kDLInt, DLDeviceType_kDLCPU,
};

pub trait Operator {
    fn init(&self, a_shape: Vec<usize>, b_shape: Vec<usize>, c_shape: Vec<usize>) -> Status;

    fn launch(&self, inputs: Vec<Tensor>, output: Tensor) -> (Status, Tensor);
}

#[derive(Debug, PartialEq)]
pub enum Status {
    Succeed,
    ValidateFailed,
    InitFailed,
    LaunchFailed,
}

#[derive(Debug, PartialEq, Clone)]
pub enum OpType {
    Add,
    Sub,
}

impl From<i32> for OpType {
    fn from(op_type: i32) -> Self {
        match op_type {
            0i32 => OpType::Add,
            1i32 => OpType::Sub,
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
                code: DLDataTypeCode_kDLInt as u8,
                bits: 32u8,
                lanes: 1u16,
            },
            DataType::INT8 => DLDataType {
                code: DLDataTypeCode_kDLInt as u8,
                bits: 8u8,
                lanes: 1u16,
            },
            DataType::FP32 => DLDataType {
                code: DLDataTypeCode_kDLFloat as u8,
                bits: 32u8,
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

    pub fn as_dltensor(&self) -> DLTensor {
        DLTensor {
            data: self.data().as_mut_ptr() as *mut c_void,
            ctx: DLContext {
                device_type: DLDeviceType_kDLCPU,
                device_id: 0 as c_int,
            },
            ndim: self.shape.len() as c_int,
            dtype: self.dtype().as_dldtype(),
            shape: self.shape().as_mut_ptr() as *mut i64,
            strides: self.strides.as_ref().unwrap().as_ptr() as *mut i64,
            byte_offset: 0,
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
            let shape =
                slice::from_raw_parts_mut(dlt.shape as *mut usize, dlt.ndim as usize).to_vec();
            let size = shape.iter().map(|v| *v as usize).product::<usize>() as usize;
            let itemsize = dlt.dtype.bits as usize / 8;
            let data = slice::from_raw_parts(dlt.data as *const u8, size * itemsize).to_vec();

            Self {
                dtype: DataType::from(dlt.dtype),
                shape,
                strides: if dlt.strides.is_null() {
                    None
                } else {
                    Some(
                        slice::from_raw_parts_mut(dlt.strides as *mut usize, dlt.ndim as usize)
                            .to_vec(),
                    )
                },
                data,
            }
        }
    }
}
