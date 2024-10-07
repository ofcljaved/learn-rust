use std::io;

fn main() {
    println!("Enter a number to check if it's even or not: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let input: u32 = input
        .trim()
        .parse()
        .expect("Give us a valid positive number");
    if is_even(input) {
        print!("The {} is even", input);
    } else {
        print!("The {} is odd", input);
    }
}

fn is_even(num: u32) -> bool {
    if (num % 2) == 0 {
        return true;
    }
    false
}
