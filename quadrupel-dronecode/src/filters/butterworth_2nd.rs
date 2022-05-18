#![feature(generic_const_exprs)]

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::fs::read_to_string;
use std::io::Write;
use std::fs::OpenOptions;

use fixed::{types, FixedI32};
use cordic::{sqrt, tan, sin};

pub type FI32 = FixedI32<types::extra::U16>;

pub struct ButterwothLowPass2nd {
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

impl ButterwothLowPass2nd {
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

    pub fn filter(&mut self, mut x: FI32) -> FI32 {
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

fn main() {
    let a_yi = FI32::from_num(4143.205);
    let a_yi_1 = FI32::from_num(8102.361)/a_yi;
    let a_yi_2 = FI32::from_num(-3963.156)/a_yi;
    let a_xi = FI32::from_num(1)/a_yi;
    let a_xi_1 = FI32::from_num(2)/a_yi;
    let a_xi_2 = FI32::from_num(1)/a_yi;

    let mut low_pass = ButterwothLowPass2nd::new(a_yi, a_yi_1, a_yi_2, a_xi, a_xi_1, a_xi_2);

    let mut output = File::create("output.txt").unwrap();
    
    let input = read_to_string("accel.txt").expect("File not found");
    let input: Vec<FI32> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    
    for x in input {
        let mut y = low_pass.filter(x);
        // println!("{}->{}", x, y);
        write!(output, "{}\n", y);
    }


}

