use std::fs;

fn main() {
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
