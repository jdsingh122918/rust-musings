/* Find the largest palindrome that is a product of two three digit numbers
 */
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut initial_result = Vec::new();
    for i in 100..999 {
        for j in 100..999 {
            let input = i * j;
            if is_palindrome(input) {
                initial_result.push(input);
            }
        }
    }
    let result = calculate_unique_values(initial_result);
    let answer: i32 = result[result.len() - 1];
    let end = start.elapsed();
    println!("Largest Palindrome: {}", answer);
    println!("Execution Time: {}", end.as_millis());
}

fn is_palindrome(input: i32) -> bool {
    let result: Vec<u32> = input
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();
    let mut reverse = result.clone();
    reverse.reverse();
    result == reverse
}

fn calculate_unique_values(mut input: Vec<i32>) -> Vec<i32> {
    input.sort();
    input.dedup();
    input
}
