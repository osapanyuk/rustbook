use std::fs;

fn main() {
    // Declare variable to store total point count.
    let mut total_pts_pt1: i64 = 0;
    let mut total_pts_pt2: i64 = 0;

    // Loop through each line in the input.txt file.
    for line in fs::read_to_string("input.txt")
        .unwrap()
        .lines() {
            // Create a switch statement for the move you played.
            if let Some((op_move, my_move)) = line.split_once(" ") {
                let ascii_my_move = my_move.chars().next().unwrap() as i64;
                let ascii_op_move = op_move.chars().next().unwrap() as i64;

                total_pts_pt1 += part_1(ascii_op_move, ascii_my_move);
                total_pts_pt2 += part_2(ascii_op_move, ascii_my_move);
            };
    };

    println!("Total points part 1 {}", total_pts_pt1);
    println!("Total points part 2 {}", total_pts_pt2);
}

fn part_1(op_move: i64, my_move: i64) -> i64 {

    let mut round_pts = my_move - 87;
    match (my_move - op_move) % 3 {
        0 => round_pts += 6,
        1 => round_pts += 0,
        2 => round_pts += 3,
        _ => println!("Something went wrong with the input.")
    };

    return round_pts
}

fn part_2(op_move: i64, my_move: i64) -> i64 {
    let mut round_pts = 0;
    match my_move {
        88 => {
            round_pts = ((op_move - 1 - 65) % 3 + 3) % 3 + 1;
            round_pts += 0;
        },
        89 => {
            round_pts = op_move - 64;
            round_pts += 3;
        },
        90 => {
            round_pts = (op_move - 64) % 3 + 1;
            round_pts += 6;
        },
        _ => println!("Something went wrong with the input.")
    };
    return round_pts;
}
