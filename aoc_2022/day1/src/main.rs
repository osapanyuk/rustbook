use std::{fs, u64};

fn main() {
    let mut current_sum: u64 = 0;
    let mut highest_sum: u64 = 0;
    for line in fs::read_to_string("input.txt")
        .unwrap()
        .lines() {

            let trimmed_line = line.trim();

            if trimmed_line == "" {
                current_sum = 0;
            } else {
                let calorie: u64 = trimmed_line
                    .parse()
                    .expect("Failed to convert to int");

                current_sum = current_sum + calorie;

                if highest_sum < current_sum {
                    highest_sum = current_sum;
                }
            }
    }

    println!("{highest_sum}")
}
