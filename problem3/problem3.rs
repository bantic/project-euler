/*
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143 ?
 */

fn divisors(x:int) -> Vec<int> {
    let mut factors: Vec<int> = vec![];
    let mut divisor = 2;
    let mut number  = x;

    loop {
        match number % divisor {
            0 => {
                factors.push(divisor);
                number = number / divisor;
            },
            _ => {
                divisor = divisor + 1;
            }
        }
        if number == 1 { break; }
    }

    factors
}

fn main() {
    let num = 600851475143;
    println!("divisors of {}: {}", num, divisors(num));
}
