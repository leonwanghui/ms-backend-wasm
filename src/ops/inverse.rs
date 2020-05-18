use super::types::OpInfo;
use nalgebra::Matrix3;

pub struct InverseOp {
    pub input: Matrix3<f32>,
}

impl InverseOp {
    pub fn new() -> InverseOp {
        InverseOp {
            input: Matrix3::new(0., 0., 0., 0., 0., 0., 0., 0., 0.),
        }
    }
}

impl OpInfo for InverseOp {
    fn init(&mut self) -> bool {
        self.input = Matrix3::new(2., 1., 1., 3., 2., 1., 2., 1., 2.);
        println!("InverseOp init success!");
        true
    }

    fn run(&self) -> i32 {
        let res: i32 = match self.input.try_inverse() {
            Some(inv) => {
                println!("The inverse of input is: {}", inv);
                let res_raw_ptr = inv.as_ptr();
                let res = res_raw_ptr as i32;
                res
            }
            None => {
                println!("Input is not invertible!");
                0
            }
        };
        println!("InverseOp run success!");
        res
    }
}
