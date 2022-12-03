fn main() {
    let input = String::from(include_str!("input"));

    let mut calories_per_elf : Vec<Option<u32>> = input.split("\n").map(|line| line.parse::<u32>().ok()).collect();
    calories_per_elf.dedup_by(|a, b| if let (Some(a), Some(b)) = (a, b) {
        *b += *a;
        true
    } else {
        false
    });

    let mut sum_calories_per_elf : Vec<u32> = calories_per_elf.iter().filter(|v| v.is_some()).map(|v| v.unwrap()).collect();

    println!("Part 1: {:?}", sum_calories_per_elf.iter().max().unwrap());

    sum_calories_per_elf.sort();
    println!("Part 2: {:?}", sum_calories_per_elf.iter().rev().take(3).sum::<u32>());
}