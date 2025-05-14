fn main() {
    let number = 10;
    println!("Given number is even: {}", is_even(number));
}

fn is_even(number: u32) -> bool {
    if number % 2 == 0 {
        true
    } else {
        false
    }
}
