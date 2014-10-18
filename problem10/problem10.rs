mod numbers;

fn main() {
    let max = 2000000;
    let mut primes = numbers::Primes::new();
    let mut sum = 0;

    for i in primes {
        if i > max { break; }
        println!("prime: {}",i);
        sum = sum + i;
    }

    println!("sum: {}",sum);
}
