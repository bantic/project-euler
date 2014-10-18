/*

The sum of the squares of the first ten natural numbers is,

1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)^2 = 552 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

 */

fn main() {
    let max = 100;
    let mut sum_numbers = 0;
    let mut sum_squares = 0;

    for i in std::iter::count(1i, 1i).take(max) {
        println!("i {}, sum_numbers: {}, sum-squres: {}",i,sum_numbers,sum_squares);
        sum_numbers = sum_numbers + i;
        sum_squares = sum_squares + (i*i);
    }

    let diff = (sum_numbers*sum_numbers) - sum_squares;
    println!("diff: {}",diff);
}
