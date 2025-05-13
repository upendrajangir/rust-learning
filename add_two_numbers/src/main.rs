fn main() {
    let adjend = -10;
    let addend = 20;
    println!("Addition of {} and {} is {}", adjend, addend, add_two_numbers(adjend, addend));
}

/// Adds two numbers and returns their sum
///
/// # Arguments
///
/// * `adjend` - The first number to add
/// * `addend` - The second number to add
///
/// # Returns
///
/// * The sum of the two input numbers
///
/// # Examples
///
/// ```
/// let result = add_two_numbers(-10, 20);
/// assert_eq!(result, 10);
/// ```
fn add_two_numbers(adjend: i32, addend: i32) -> i32 {
    adjend + addend
}