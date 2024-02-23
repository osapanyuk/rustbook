use std::fs;
use std::time::Instant;
use std::collections::HashMap;

fn final_solution_part1() {

    let sum = include_bytes!("../input.txt")
        .split(|byte| *byte == b'\n')
        .map(|line| line.split_at(line.len() / 2))
        .map(|(cont1, cont2)| cont1
             .iter()
             .filter(|ch| cont2.contains(ch))
             .map(|matched| if *matched >= b'a' {
                 return (*matched - b'a') as usize + 1;
             } else {
                 return (*matched - b'A') as usize + 27;
             })
             .next()
             .unwrap())
        .sum::<usize>();

    println!("{sum}");
}

fn solution_1() {
    let answ: u64 = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.as_bytes())
        .map(|line| line[..line.len()/2].iter().filter(|ch| line[line.len()/2..].contains(ch)).collect::<Vec<_>>())
        .map(|ch| match ch[0] {
            b'a'..=b'z' => *ch[0] as u64 - b'a' as u64 + 1,
            b'A'..=b'Z' => *ch[0] as u64 - b'A' as u64 + 27,
            _ => 0
        })
        .sum();
    println!("{}", answ);
}

fn solution_2() {
    let mut sum: usize = 0;
    for line in fs::read_to_string("input.txt")
        .unwrap()
        .lines() {

            let mut hmap = HashMap::new();
            let (cont1, cont2) = line.split_at(line.len()/2);
            for ch in cont2.chars() {
                hmap.insert(ch, 1);
            };
            let mut matching_ch = ' ';
            for ch in cont1.chars() {
                match hmap.get(&ch) {
                    Some(_) => {
                        matching_ch = ch;
                        break;
                    },
                    _ => continue
                }
            };

            sum += match matching_ch {
                'a'..='z' => matching_ch as usize - b'a' as usize + 1,
                'A'..='Z' => matching_ch as usize - b'A' as usize + 27,
                _ => 0
            };
        };
    println!("{sum}");
}

fn main() {
    let now = Instant::now();

    solution_1();

    let elapsed = now.elapsed();
    println!("Solution 1 time: {:.2?}", elapsed);

    let now = Instant::now();

    solution_2();

    let elapsed = now.elapsed();
    println!("Solution 2 time: {:.2?}", elapsed);

    let now = Instant::now();

    final_solution_part1();

    let elapsed = now.elapsed();
    println!("Final Solution time: {:.2?}", elapsed);
}
