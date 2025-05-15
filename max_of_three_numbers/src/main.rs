fn main() {
    let first_number: i32 = -3;
    let second_number: i32 = 3;
    let third_number: i32 = 11;
    // println!("{}", max_of_three_numbers(first_number, second_number, third_number));

    println!("{}", std::cmp::max(first_number, std::cmp::max(second_number, third_number)));
}

fn max_of_three_numbers(first_number: i32, second_number: i32, third_number: i32) -> i32 {
    if first_number >= second_number && first_number >= third_number {
        first_number
    } else if second_number >= first_number && second_number >= third_number {
        second_number
    } else {
        third_number
    }
}