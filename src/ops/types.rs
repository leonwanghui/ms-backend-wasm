use std::boxed::Box;
use std::convert::From;

pub trait Operator {
    fn init(&mut self, number_type: NumberType) -> OpStatus;

    fn launch(&self, inputs: Vec<Box<Vec<NumberType>>>) -> (OpStatus, Vec<Box<Vec<NumberType>>>);
}

#[derive(Debug, PartialEq)]
pub enum OpType {
    Add = 0,
    Mul,
    Argmax,
    EqualCount,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum NumberType {
    FP32(f32),
    INT8(i8),
    DEFAULT(usize),
}

impl NumberType {
    pub fn cast_f32(&self) -> f32 {
        if let NumberType::FP32(c) = *self {
            c
        } else {
            unreachable!()
        }
    }

    pub fn cast_i8(&self) -> i8 {
        if let NumberType::INT8(c) = *self {
            c
        } else {
            unreachable!()
        }
    }
}

impl From<f32> for NumberType {
    fn from(num: f32) -> Self {
        NumberType::FP32(num)
    }
}

impl From<i8> for NumberType {
    fn from(num: i8) -> Self {
        NumberType::INT8(num)
    }
}

impl From<usize> for NumberType {
    fn from(num: usize) -> Self {
        NumberType::DEFAULT(num)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum OpStatus {
    Succeed,
    InitFailed,
    LaunchFailed,
}
