use std::{io, u64};

fn main() {
    // Get user input to determine which fibonacci number to output
    println!("Please indicate which fibonacci number to count to by position");

    let position: u64 = loop {
        let mut position = String::new();

        io::stdin()
            .read_line(&mut position)
            .expect("Error when reading input");

        match position.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a valid positive integer")
        };
    };

    // Call the fibonacci function
    println!("{}", fibonacci(position));
}

fn fibonacci(number: u64) -> u64 {
    // Generate the base cases
    let mut prev2: u64 = 0;
    let mut prev: u64 = 1;

    if number == 0 {
        return prev2;
    };

    if number == 1 {
        return prev;
    };

    // Loop through and sum the previous 2 values  until the value is reached
    let mut fib = 0;
    for _count in 2..number + 1 {
        fib = prev2 + prev;
        prev2 = prev;
        prev = fib;
    };

    return fib;
}
