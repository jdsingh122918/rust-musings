/*
If we list all the numbers below 10 that are multiples of of 3 or 5, we get 3, 5, 6 and 9.
The sum of these numbers is 23 (3 + 5 + 6 + 9). Find the sum of all the multiples of 3 or 5
below 1000
*/

use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("The answer is: ");
    let mut sum = 0;
    for input in 1 .. 1000 {
        let result = is_divisible(input);
        if result {
            sum += input
        }
    }
    let end = start.elapsed();
    println!("{}", sum);
    println!("execution time: {}", end.as_nanos());
}

/*
This function takes an integer as a parameter and returns the integer if
it is divisible by 3 or 5
*/
fn is_divisible(input: i32) -> bool {
    if input % 15 == 0 {
        true
    } else if input % 5 == 0 {
        true
    } else if input % 3 == 0 {
        true
    } else {
        false
    }
}
