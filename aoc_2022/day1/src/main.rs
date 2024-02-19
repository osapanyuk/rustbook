use std::{fs, u64};

fn main() {
    let mut current_sum: u64 = 0;
    let mut first_sum: u64 = 0;
    let mut second_sum: u64 = 0;
    let mut third_sum: u64 = 0;
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

                if first_sum < current_sum {
                    third_sum = second_sum;
                    second_sum = first_sum;
                    first_sum = current_sum;
                } else if second_sum < current_sum {
                    third_sum = second_sum;
                    second_sum = current_sum;
                } else if third_sum < current_sum {
                    third_sum = current_sum;
                }
            }
    }

    println!("First sum: {first_sum}");
    println!("Second sum: {second_sum}");
    println!("Third sum: {third_sum}");
    println!("Final sum: {}", first_sum + second_sum + third_sum);
}
