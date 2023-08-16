/*
Find the sum of even valued fibonacci numbers less than 4000000
*/
use std::time::Instant;

fn main() {
    let start: Instant = Instant::now();
    const UPPER_LIMIT: i32 = 40000000;
    let mut sum = 0;
    let mut count = 1;
    loop {
        let result = calculate_fibonacci(count);
        if result < UPPER_LIMIT {
            if &result % 2 == 0 {
                println!("{}", result);
                sum += &result;
            }
            count += 1;
            continue;
        } else {
            break;
        }
    }
    let end = start.elapsed();
    println!("sum: {}", sum);
    println!("Execution time: {}", end.as_nanos());
}

fn calculate_fibonacci(input: i32) -> i32 {
    match input {
        1 => 1,
        2 => 2,
        _ => calculate_fibonacci(&input - 1) + calculate_fibonacci(&input - 2),
    }
}
