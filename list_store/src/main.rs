use std::io;

fn main() {
    println!("Welcome to the interactive list application.");

    let mut my_list = Vec::new();

    loop {
        println!("Select one of the following options");
        println!("1. Insert string into list");
        println!("2. Print list");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Something went wrong when reading input.");

        let option: u32 = match option.trim().parse() {
            Ok(opt) => opt,
            Err(_) => continue,
        };

        match option {
            1 => {
                println!("Please enter the string to save.");
                let mut insert = String::new();

                io::stdin()
                    .read_line(&mut insert)
                    .expect("Something went wrong when reading input.");

                my_list.push(insert.trim().to_string());
            },
            2 => {
                for element in &my_list {
                    println!("{},", element);
                }
            },
           _ => continue,
        }
    }
}
