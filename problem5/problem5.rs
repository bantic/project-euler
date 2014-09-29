/*
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 *
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */

// just did this by thinking about it:

fn main() {
    //need: // 2 3 5 7 11 13 17 19
    //add: // 2
    //add: // 2
    //add: // 2
    //add: // 3

    //get: // 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20

    let result: int = 2 * 2 * 2 * 2 *
                      3 * 3 *
                      5 * 7 * 11 * 13 * 17 * 19;
    println!("Result: {}",result);
}
