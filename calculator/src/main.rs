use std::io;

fn main() {
    let operation = loop {
        let mut operation = String::new();

        io::stdin().read_line(&mut operation).expect("Failed reading input");

        match operation.trim() {
            "+" => println!("Addition"),
            "-" => println!("Substraction"),
            "*" => println!("Multiplication"),
            "/" => println!("Division"),
            _ => println!("Invalid Input")
        };
        break operation;

    };

    println!("{operation}");
}
