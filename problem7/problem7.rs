/*
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 */

mod numbers;

fn main() {
    let mut count = 0i;
    let mut current = 2i;
    let max = 10001;
    let mut primes: Vec<int> = vec![];
    loop {
        if numbers::Prime::is_prime(current) {
            primes.push(current);
            count = count + 1;

            if count == max {
                println!("prime #{}: {}",count,current);
                break;
            }

            //if count % 1000 == 0 {
                println!("prime #{}: {} ",count, current);
            //}

        }
        current = current + 1;
    }
}
