use std::iter::range_step;

#[path = "/Users/coryforsyth/work/experiments/project-euler/common/numbers.rs"]
mod numbers;

fn check_a_b(a:int, b:int) -> int {
    let mut n = 0i;
    loop {
        let val = n*n + a*n + b;
        if !numbers::Primes::is_prime(val) { break; }
        n += 1;
    }

    n
}


fn main() {
    let mut max = 0i;
    let mut max_a = 0i;
    let mut max_b = 0i;
    for a in range_step(-999i, 1000i, 1i) {
        for b in range_step(-999i, 1000i, 1i) {
            let consecutive = check_a_b(a,b);
            if consecutive > max {
                max = consecutive;
                max_a = a;
                max_b = b;
            }
        }
    }

    println!("a {}, b {} -> {} primes",max_a,max_b,max);
}
