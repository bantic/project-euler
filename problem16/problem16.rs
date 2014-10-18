/*
 * 2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.
 *
 * What is the sum of the digits of the number 2^1000?
 */
extern crate num;

use num::bigint::{ToBigInt};
use std::num;

fn main() {
    let base = 2i.to_bigint().unwrap();
    let num = num::pow(base, 1000);
    let s = num.to_str();
    let mut _s = s.as_slice();
    let mut sum = 0;

    println!("num: {}",num);

    for digit_s in _s.chars() {
        let digit_n = digit_s.to_digit(10);
        //println!("{} -> {}",digit_s,digit_n);
        match digit_n {
            Some(number) => {
                sum = sum + number;
            },
            _ => {
                println!("Skipping: {}",digit_n);
            }
        }
    }
    println!("Sum! {}",sum);
}
