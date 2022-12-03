fn get_score_part1(line : &str) -> u32 {
    if line.is_empty() {
        return 0;
    }

    let splitted : Vec<&str> = line.split_whitespace().collect();
    match (splitted[0], splitted[1]) {
        ("A", "X") => 1 + 3,
        ("A", "Y") => 2 + 6,
        ("A", "Z") => 3 + 0,
        ("B", "X") => 1 + 0,
        ("B", "Y") => 2 + 3,
        ("B", "Z") => 3 + 6,
        ("C", "X") => 1 + 6,
        ("C", "Y") => 2 + 0,
        ("C", "Z") => 3 + 3,
        _ => panic!("Invalid input {:?}", splitted)
    }
}

fn get_score_part2(line : &str) -> u32 {
    if line.is_empty() {
        return 0;
    }

    let splitted : Vec<&str> = line.split_whitespace().collect();
    match (splitted[0], splitted[1]) {
        ("A", "X") => 0 + 3,
        ("A", "Y") => 3 + 1,
        ("A", "Z") => 6 + 2,
        ("B", "X") => 0 + 1,
        ("B", "Y") => 3 + 2,
        ("B", "Z") => 6 + 3,
        ("C", "X") => 0 + 2,
        ("C", "Y") => 3 + 3,
        ("C", "Z") => 6 + 1,
        _ => panic!("Invalid input {:?}", splitted)
    }
}

fn main() {
    let input = include_str!("input");

    let score : u32 = input.split("\n").map(get_score_part1).sum();
    println!("Part 1: {}", score);

    let score2 : u32 = input.split("\n").map(get_score_part2).sum();
    println!("Part 2: {}", score2);
}
