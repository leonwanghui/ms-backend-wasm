use super::types::OpInfo;
use ndarray::{arr2, Array2};

pub struct AddOp {
    pub input_a: Array2<i32>,
    pub input_b: Array2<i32>,
}

impl AddOp {
    pub fn new() -> AddOp {
        AddOp {
            input_a: Array2::<i32>::zeros((2, 2)),
            input_b: Array2::<i32>::zeros((2, 2)),
        }
    }
}

impl OpInfo for AddOp {
    fn init(&mut self) -> bool {
        self.input_a = arr2(&[[1, 2], [3, 4]]);
        self.input_b = arr2(&[[5, 6], [7, 8]]);
        println!("AddOp init success!");
        true
    }

    fn run(&self) -> i32 {
        let z = &self.input_a + &self.input_b;
        println!("AddOp result is {}", z);
        println!("AddOp run success!");
        let res_raw_ptr = z.as_ptr();
        let res = res_raw_ptr as i32;
        res
    }
}
