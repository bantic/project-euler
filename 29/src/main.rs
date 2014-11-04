extern crate num;
extern crate core;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use std::num::{One};

fn big_pow(base:uint, exp:uint) -> BigUint {
    let mut result:BigUint = One::one();
    let big_base:BigUint = base.to_biguint().unwrap();
    let mut reps = exp;

    while reps > 0 {
        result = big_base * result;
        reps -= 1;
    }


    result
}

fn main() {
    let mut results:Vec<BigUint> = vec![];
    for base in std::iter::range_inclusive(2u, 100u) {
        for exp in std::iter::range_inclusive(2u, 100u) {
            let result = big_pow(base, exp);
            if results.contains(&result) {
                //println!("skipping {}^{}={}",base,exp,result);
            } else {
                //println!("adding {}^{}={}",base,exp,result);
                results.push(result);
            }
        }
    }
    println!("total distinct: {}",results.len());
}
