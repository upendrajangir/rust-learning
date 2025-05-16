use std::io::stdin;

fn main() {
    let mut first_number: String = String::new();
    let mut second_number: String = String::new();
    let mut operation: String = String::new();

    println!("Enter the first number: ");
    stdin().read_line(&mut first_number).expect("Failed to read line");
    let first_number: i32 = first_number.trim().parse().expect("Please type a number!");

    println!("Enter the operation: ");
    stdin().read_line(&mut operation).expect("Failed to read line");
    let operation: char = operation.trim().parse().expect("Please type a valid operation!");    

    println!("Enter the second number: ");
    stdin().read_line(&mut second_number).expect("Failed to read line");
    let second_number: i32 = second_number.trim().parse().expect("Please type a number!");

    match operation {
        '+' => println!("The result of adding the two numbers is: {}", add_two_numbers(first_number, second_number)),
        '-' => println!("The result of subtracting the two numbers is: {}", subtract_two_numbers(first_number, second_number)), 
        '*' => println!("The result of multiplying the two numbers is: {}", multiply_two_numbers(first_number, second_number)),
        '/' => println!("The result of dividing the two numbers is: {}", divide_two_numbers(first_number, second_number)),
        _ => println!("Invalid operation! Please use +, -, *, or /"),
    }
}

fn add_two_numbers(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn subtract_two_numbers(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

fn multiply_two_numbers(first_number: i32, second_number: i32) -> i32 {
    first_number * second_number
}

fn divide_two_numbers(first_number: i32, second_number: i32) -> i32 {
    first_number / second_number
}


