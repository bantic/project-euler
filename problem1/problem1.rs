/*
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn main() {
    let mut sum = 0;
    for i in range(0i,1000) {
        println!("{}",i%3);
        match i%3 {
            0 => {
                sum = sum + i;
            },
            _ => {
                match i%5 {
                    0 => {
                        sum = sum + i;
                    },
                    _ => {}
                }
            }
        }
    }
    println!("sum: {}",sum);
}
