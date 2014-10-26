use std::collections::HashMap;
use std::collections::hashmap::{Vacant,Occupied};
use std::iter::{range_inclusive};
pub struct ProperDivisorCounts { map: HashMap<uint, uint> }
impl ProperDivisorCounts {
    pub fn new() -> ProperDivisorCounts {
        let map = HashMap::new();
        ProperDivisorCounts { map: map }
    }

    pub fn of(&mut self, num:uint) -> &uint {
        let result = match self.map.entry(num) {
            Vacant(entry)   => entry.set(proper_divisor_sum(num)),
            Occupied(entry) => entry.into_mut()
        };

        result
    }
}

fn proper_divisor_sum(num: uint) -> uint {
    proper_divisors(num).iter().fold(0u, |s,x| s + *x)
}

// proper divisors are numbers less than n that divide evenly,
// so don't include n. I.e., proper_divisors of 10 = [1,2,5],
// not [1,2,5,10]
fn proper_divisors(num: uint) -> Vec<uint> {
    let mut factors: Vec<uint> = vec![];
    let sqrt  = num.to_f64().unwrap().sqrt().floor();

    for i in range_inclusive(1,sqrt.to_uint().unwrap()) {
        if num % i == 0 {
            if !factors.contains(&i) {
                factors.push(i);
            }
            let div = num / i;
            if !factors.contains(&div) && div != num {
                factors.push(div);
            }
        }
    }
    factors
}

fn main() {
    let max = 10000u;
    let mut amicables:Vec<uint> = vec![];
    let mut amicable_pairs = vec![];

    let mut divisors = ProperDivisorCounts::new();
    for a in range(2u, max) {
        let b = *divisors.of(a);
        println!("{} -> {}",a,b);
        if b > max { continue; }
        if amicables.contains(&b) { continue; }

        if a != b && *divisors.of(b) == a {
            amicables.push(a);
            amicables.push(b);
            amicable_pairs.push( [a,b] );
        }
    }

    let sum = amicables.iter().fold(0u, |s,x| s + *x);
    println!("amicables: {}, sum: {}",amicables,sum);
}
