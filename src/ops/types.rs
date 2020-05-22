use std::boxed::Box;
use std::convert::From;

pub trait Operator {
    fn init(&mut self, data_type: i32, shape: Vec<usize>, dim_size: usize) -> OpStatus;

    fn launch(&self, inputs: Vec<Box<Tensor>>) -> (OpStatus, Vec<Box<Tensor>>);
}

#[derive(Debug, PartialEq)]
pub enum OpType {
    Add = 0,
    Mul,
    Argmax,
    EqualCount,
}

#[derive(Debug, PartialEq)]
pub enum DataType {
    BooleanType = 0,
    NumericType,
    OneDArrayType,
    TwoDArrayType,
    ThreeDArrayType,
}

#[derive(Debug, PartialEq)]
pub enum OpStatus {
    Succeed,
    InitFailed,
    LaunchFailed,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Tensor {
    Boolean(bool),
    Numeric(f32),
    OneDArray(Vec<f32>),
    TwoDArray(Vec<Vec<f32>>),
    ThreeDArray(Vec<Vec<Vec<f32>>>),
    Default(usize),
}

impl Tensor {
    pub fn cast_bool(&self) -> bool {
        if let Tensor::Boolean(c) = *self {
            c
        } else {
            unreachable!()
        }
    }

    pub fn cast_f32(&self) -> f32 {
        if let Tensor::Numeric(c) = *self {
            c
        } else {
            unreachable!()
        }
    }

    pub fn cast_1d_array(&self) -> Vec<f32> {
        if let Tensor::OneDArray(c) = &*self {
            c.to_vec()
        } else {
            unreachable!()
        }
    }

    pub fn cast_2d_array(&self) -> Vec<Vec<f32>> {
        if let Tensor::TwoDArray(c) = &*self {
            c.to_vec()
        } else {
            unreachable!()
        }
    }

    pub fn cast_3d_array(&self) -> Vec<Vec<Vec<f32>>> {
        if let Tensor::ThreeDArray(c) = &*self {
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

impl From<f32> for Tensor {
    fn from(data: f32) -> Self {
        Tensor::Numeric(data)
    }
}

impl From<Vec<f32>> for Tensor {
    fn from(data: Vec<f32>) -> Self {
        Tensor::OneDArray(data)
    }
}

impl From<Vec<Vec<f32>>> for Tensor {
    fn from(data: Vec<Vec<f32>>) -> Self {
        Tensor::TwoDArray(data)
    }
}

impl From<Vec<Vec<Vec<f32>>>> for Tensor {
    fn from(data: Vec<Vec<Vec<f32>>>) -> Self {
        Tensor::ThreeDArray(data)
    }
}

impl From<usize> for Tensor {
    fn from(data: usize) -> Self {
        Tensor::Default(data)
    }
}

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub enum NumberType {
//     FP32(f32),
//     INT8(i8),
//     DEFAULT(usize),
// }

// impl NumberType {
//     pub fn cast_f32(&self) -> f32 {
//         if let NumberType::FP32(c) = *self {
//             c
//         } else {
//             unreachable!()
//         }
//     }

//     pub fn cast_i8(&self) -> i8 {
//         if let NumberType::INT8(c) = *self {
//             c
//         } else {
//             unreachable!()
//         }
//     }
// }

// impl From<f32> for NumberType {
//     fn from(num: f32) -> Self {
//         NumberType::FP32(num)
//     }
// }

// impl From<i8> for NumberType {
//     fn from(num: i8) -> Self {
//         NumberType::INT8(num)
//     }
// }

// impl From<usize> for NumberType {
//     fn from(num: usize) -> Self {
//         NumberType::DEFAULT(num)
//     }
// }
