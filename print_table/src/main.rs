fn main() {
    print_table_of(9);
}

// prints the table of a given number
//
// # Arguments
//
// * `number` - The number to print the table of
//
// # Returns
//
// * None
//
// # Examples
//
// ```
// print_table_of(9);
// ```
fn print_table_of(number: u32) {
    for i in 1..=10 {
        println!("{} * {} = {}", number, i, number * i);
    }
}