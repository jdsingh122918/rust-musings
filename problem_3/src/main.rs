/* Find the largest prime factor given an input
 */
fn main() {
    let result = calculate_prime_factors(100);
    println!("{:?}", result);
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
