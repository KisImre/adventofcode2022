// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn part12(input: &str) -> (u32, u32) {
    let mut calories_per_elf: Vec<Option<u32>> = input
        .split("\n")
        .map(|line| line.parse::<u32>().ok())
        .collect();
    calories_per_elf.dedup_by(|a, b| {
        if let (Some(a), Some(b)) = (a, b) {
            *b += *a;
            true
        } else {
            false
        }
    });

    let mut sum_calories_per_elf: Vec<u32> = calories_per_elf
        .iter()
        .filter(|v| v.is_some())
        .map(|v| v.unwrap())
        .collect();

    sum_calories_per_elf.sort();
    sum_calories_per_elf.reverse();

    (
        sum_calories_per_elf[0],
        sum_calories_per_elf.iter().take(3).sum::<u32>(),
    )
}

fn main() {
    assert_eq!((24000, 45000), part12(include_str!("input_test")));

    let (part1, part2) = part12(include_str!("input"));

    println!("Day 01");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
