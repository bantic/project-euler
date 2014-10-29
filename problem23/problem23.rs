use std::collections::HashMap;
use std::collections::hashmap::{Vacant,Occupied};
use std::iter::{range_inclusive};
pub struct ProperDivisors { map: HashMap<uint, Vec<uint>> }
impl ProperDivisors {
    fn new() -> ProperDivisors {
        ProperDivisors { map: HashMap::new() }
    }

    fn _of(num:uint) -> Vec<uint> {
        let mut factors: Vec<uint> = vec![];
        let sqrt  = num.to_f64().unwrap().sqrt().floor();

        for i in range_inclusive(1,sqrt.to_uint().unwrap()) {
            if num % i == 0 {
                if !factors.contains(&i) { factors.push(i); }

                let div = num / i;
                if !factors.contains(&div) && div != num {
                    factors.push(div);
                }
            }
        }
        factors
    }

    fn sum(&mut self, num:&uint) -> uint {
        let mut sum = 0u;
        for d in self.of(*num).iter() {
            sum += *d;
        }
        sum
    }

    fn is_abundant(&mut self, num:uint) -> bool {
        self.sum(&num) > num
    }

    fn of(&mut self, num:uint) -> &mut Vec<uint> {
        let result = match self.map.entry(num) {
            Vacant(entry)   => entry.set(ProperDivisors::_of(num)),
            Occupied(entry) => entry.into_mut()
        };

        result
    }
}

fn main() {
    let max = 28123;
    let mut divisors = ProperDivisors::new();
    let mut abundants = vec![];
    for i in range(1u,max) {
        if divisors.is_abundant(i) {
            abundants.push(i);
        }
    }
    let mut summables = vec![];
    let mut non_summables = vec![];

    for i in range(1u, max) {
        let mut is_sum = false;
        for a in abundants.iter() {
            let val = *a;
            if val > i { break; }
            let diff = i - val;
            if divisors.is_abundant(diff) {
                //println!("{} -> {}+{}",i,a,diff);
                is_sum = true;
                break;
            }
        }
        if is_sum {
            summables.push(i);
        } else {
            non_summables.push(i);
        }
    }

    println!("non-summable: {}",non_summables.iter().fold(0u, |s,x| s+*x));
}
