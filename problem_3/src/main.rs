/* Find the largest prime factor given an input
 */use std::time::{Instant};

fn main() {
    let start = Instant::now();
    let input = 600851475143;
    let result = calculate_prime_factors(input);
    let unique_result = calculate_unique_values(result);
    let end = start.elapsed();
    println!("Largest Prime Factor of {} is {}", input, unique_result[unique_result.len() - 1]);
    println!("Execution Time: {}", end.as_millis());
}

fn calculate_prime_factors(mut input: i64) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    while input % 2 == 0 {
        result.push(2);
        input /= 2;
    }
    let ending_range: i64 = ((input as f64).sqrt()) as i64 + 1;
    for i in 3..ending_range {
        loop {
            if input % i == 0 {
                result.push(i as i32);
                input /= i;
            } else {
                break;
            }
        }
    }
    if input > 2 {
        result.push(input as i32)
    }
    result
}

fn calculate_unique_values(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input.dedup();
    input
}
