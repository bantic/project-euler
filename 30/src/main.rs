extern crate num;
use std::num::pow;

fn digits(num:uint) -> Vec<uint> {
    let mut result = vec![];
    let mut n = num;

    while n > 0 {
        result.push( n % 10 );
        n = n / 10;
    }

    result
}

fn main() {
    println!("3^5 {}",pow(3u,5));
    let mut results = vec![];

    for i in range(10u, 194980u) {
        let d = digits(i);
        let sum = d.iter().fold(0, |a, &b| a + pow(b,5));

        if i == sum {
            results.push(i);
            println!("got one: {}:{}",i,sum);
        } else {
            //println!("{} != {}",i,sum);
        }
    }

    println!("sum: {}", results.iter().fold(0, |a,&b| a+b));
}
