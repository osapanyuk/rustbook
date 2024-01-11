use core::f64;
use std::io;

fn main() {

    println!("Select from the following options:");
    println!("1. Convert celsius to fahrenheit");
    println!("2. Convert fahrenheit to celsius");

    let option: u8 = loop {

        // Prompt user for the direction of conversion
        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Error when reading user input");

        // Convert the user option to an int and match it to confirm that it is a valid option
        match option.trim().parse() {
            Ok(1) => break 1,
            Ok(2) => break 2,
            Ok(_) => {
                println!("Please input only a number corresponding to the avaiable options");
                continue;
            },
            Err(_) => {
                println!("Please input a number corresponding to the available options");
                continue;
            }
        };
    };

    println!("Please input the temperature to convert");

    loop {
        let mut temperature = String::new();

        io::stdin()
            .read_line(&mut temperature)
            .expect("Error when reading user input");

        // Convert the user input to a float
        let temperature: f64 = match temperature.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("Please input a valid integer or decimal number");
                continue;
            }
        };

        let converted_temp = match option {
            1 => convert_c_to_f(temperature),
            2 => convert_f_to_c(temperature),
            _ => break
        };

        println!("The converted temperature is {converted_temp}");
        break;
    }
}

fn convert_c_to_f(celsius: f64) -> f64 {
    return celsius * 1.8 + 32.0;
}

fn convert_f_to_c(fahrenheit: f64) -> f64 {
    return (fahrenheit - 32.0) * 5.0 / 9.0;
}
