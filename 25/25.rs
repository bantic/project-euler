extern crate num;
use num::bigint::{BigUint};
use std::num::{One, Zero};

struct BigFibonacci { cur: BigUint, prev: BigUint }
impl BigFibonacci {
    fn new() -> BigFibonacci {
        BigFibonacci { cur: Zero::zero(), prev: Zero::zero() }
    }
}

impl Iterator<BigUint> for BigFibonacci {
    fn next(&mut self) -> Option<BigUint> {
        if self.cur.is_zero() {
            self.cur = One::one();
        } else {
            let cur  = self.cur.clone();
            let prev = self.prev.clone();
            self.cur  = cur + prev;
            self.prev = cur;
        }

        Some(self.cur.clone())
    }
}

fn main() {
    let mut fib = BigFibonacci::new();
    let digits = 1000u;

    let mut idx = 1u;
    for i in fib {
        // println!("{}: {}",idx,i);

        if i.to_string().len() == digits {
            println!("stopped at F({})",idx);
            break;
        }
        idx += 1;
    }
}
