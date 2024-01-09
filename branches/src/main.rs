use std::io;

fn main() {
    if_test();
    if_in_let();
}

fn if_test() {
    let mut input = String::new();

    println!("Please input a number");

    io::stdin()
        .read_line(&mut input)
        .expect("Issue with reading input");

    let input: i32 = input.trim().parse().expect("Input is not a number");

    if input < 5 {
        println!("Condition is true. The provided input is smaller than 5");
    } else if input == 5 {
        println!("Second condition is true. The provided input is equal to 5");
    } else {
        println!("Condition is false. The provided input is not smaller or equal to 5");
    }
}

fn if_in_let() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is {number}");
}
