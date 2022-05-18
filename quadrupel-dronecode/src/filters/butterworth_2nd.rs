#![feature(generic_const_exprs)]

use fixed::{types, FixedI32};
use crate::library::fixed_point::FI32;

pub struct ButterworthLowPass2nd {
    pub a_yi: FI32,
    pub a_yi_1: FI32,
    pub a_yi_2: FI32,
    pub a_xi: FI32,
    pub a_xi_1: FI32,
    pub a_xi_2: FI32,
    pub yi_1: FI32,
    pub yi_2: FI32,
    pub xi_1: FI32,
    pub xi_2: FI32,
    
}

impl ButterworthLowPass2nd {
    pub fn new(a_yi: FI32, a_yi_1: FI32, a_yi_2: FI32, a_xi: FI32, a_xi_1: FI32, a_xi_2: FI32) -> Self {

        let yi_1 = FI32::from_num(0);
        let yi_2 = FI32::from_num(0);
        let xi_1 = FI32::from_num(0);
        let xi_2 = FI32::from_num(0);

        Self {
            a_yi,
            a_yi_1,
            a_yi_2,
            a_xi,
            a_xi_1,
            a_xi_2,
            yi_1,
            yi_2,
            xi_1,
            xi_2,
        }
    }

    pub fn filter(&mut self, x: FI32) -> FI32 {
        let c_xi = self.a_xi*x;
        let c_xi_1 = self.a_xi_1*self.xi_1;
        let c_xi_2 = self.a_xi_2*self.xi_2;
        let c_yi_1 = self.a_yi_1*self.yi_1;
        let c_yi_2 = self.a_yi_2*self.yi_2;


        let y = (c_xi+c_xi_1+c_xi_2+c_yi_1+c_yi_2);

        self.yi_2 = self.yi_1;
        self.yi_1 = y;
        self.xi_2 = self.xi_1;
        self.xi_1 = x;

        y
    }

    // fn something(&self, u: i32) -> {
        
    // }
}

// fn main() {
//     let a_yi = FI32::from_num(4143.205);
//     let a_yi_1 = FI32::from_num(8102.361)/a_yi;
//     let a_yi_2 = FI32::from_num(-3963.156)/a_yi;
//     let a_xi = FI32::from_num(1)/a_yi;
//     let a_xi_1 = FI32::from_num(2)/a_yi;
//     let a_xi_2 = FI32::from_num(1)/a_yi;
//
//     let mut low_pass = ButterwothLowPass2nd::new(a_yi, a_yi_1, a_yi_2, a_xi, a_xi_1, a_xi_2);
//
//     let mut output = File::create("output.txt").unwrap();
//
//     let input = read_to_string("accel.txt").expect("File not found");
//     let input: Vec<FI32> = input
//         .lines()
//         .map(|line| line.parse().unwrap())
//         .collect();
//
//     for x in input {
//         let mut y = low_pass.filter(x);
//         // println!("{}->{}", x, y);
//         write!(output, "{}\n", y);
//     }
//
//
// }

