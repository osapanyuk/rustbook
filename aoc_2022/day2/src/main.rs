use std::fs;

fn main() {
    // Declare variable to store total point count.
    let mut total_pts: u64 = 0;

    // Loop through each line in the input.txt file.
    for line in fs::read_to_string("input.txt")
        .unwrap()
        .lines() {
            // Create a switch statement for the move you played.
            if let Some((op_move, my_move)) = line.split_once(" ") {
                let ascii_my_move = my_move.chars().next().unwrap() as u64;
                let ascii_op_move = op_move.chars().next().unwrap() as u64;

                total_pts += ascii_my_move - 87;
                match (ascii_my_move - ascii_op_move) % 3 {
                    0 => total_pts += 6,
                    1 => total_pts += 0,
                    2 => total_pts += 3,
                    _ => println!("Something went wrong with the input.")
                };
            };
    };

    println!("Total points {}", total_pts)
}
