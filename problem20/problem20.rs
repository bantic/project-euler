
extern crate num;
use num::bigint::{BigUint, ToBigUint};
use std::num::{One, Zero, ToStrRadix};
//use std::num::{ToPrimitive, FromPrimitive};
//use std::num::{Zero, One, ToStrRadix, FromStrRadix};

fn factorial(n:BigUint) -> BigUint {
    let one = 1u.to_biguint().unwrap();

    if n.is_zero() {
        one
    } else {
        n * factorial( n - One::one() )
    }
}

fn main() {
    let num = 100u.to_biguint().unwrap();
    let fac = factorial(num);
    let str = fac.to_str_radix(10);
    let mut sum = 0u;

    let mut digits = str.as_slice().chars();

    for d in digits {
        sum = sum + d.to_digit(10).unwrap();
    }

    println!("sum {}",sum);
}
