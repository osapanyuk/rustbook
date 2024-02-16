use std::io;

fn main() {
    println!("Please input the number to reach for fizz buzz.");

    let input : u64 = loop {

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read input");

        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => println!("Please input a valid positive integer")
        };
    };
    for i in 1..input + 1 {
        let mut output = String::new();

        if i % 5 == 0 {
            output.push_str("fizz");
        };

        if i % 3 == 0 {
            output.push_str("buzz");
        };
        if output.is_empty() {
            output += &i.to_string();
        };
        println!("{output}");
    };
}
