#![feature(iter_array_chunks)]

fn get_value(c : char) -> u32 {
    if c.is_lowercase() {
        ((c as u8) - ('a' as u8) + 1) as u32
    } else {
        ((c as u8) - ('A' as u8) + 27) as u32
    }
}

fn main() {
    let input = include_str!("input");

    let mut sum1 : u32 = 0;
    for line in input.split_whitespace() {
        let (first_half, second_half) = line.split_at(line.len() / 2);

        let mut duplicates = String::new();

        for c in first_half.chars() {
            if second_half.contains(c) && !duplicates.contains(c) {
                duplicates.push(c);
            }
        }

        sum1 += duplicates.chars().map(get_value).sum::<u32>();
    }

    println!("Part 1: {}", sum1);

    let mut sum2 : u32 = 0;
    for [l1, l2, l3] in input.split_whitespace().array_chunks() {
        sum2 += l1.chars().filter(|c| l2.contains(*c) && l3.contains(*c)).map(get_value).take(1).sum::<u32>();
    }

    println!("Part 2: {}", sum2);
}
