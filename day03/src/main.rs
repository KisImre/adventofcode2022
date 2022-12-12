// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

#![feature(iter_array_chunks)]

fn get_value(c: char) -> u32 {
    if c.is_lowercase() {
        ((c as u8) - ('a' as u8) + 1) as u32
    } else {
        ((c as u8) - ('A' as u8) + 27) as u32
    }
}

fn part12(input: &str) -> (u32, u32) {
    (
        input
            .split("\n")
            .map(|line| {
                let (first_half, second_half) = line.split_at(line.len() / 2);

                let mut duplicates = String::new();

                for c in first_half.chars() {
                    if second_half.contains(c) && !duplicates.contains(c) {
                        duplicates.push(c);
                    }
                }

                duplicates.chars().map(get_value).sum::<u32>()
            })
            .sum(),
        input
            .split_whitespace()
            .into_iter()
            .array_chunks()
            .map(|[l1, l2, l3]| {
                l1.chars()
                    .filter(|c| l2.contains(*c) && l3.contains(*c))
                    .map(get_value)
                    .take(1)
                    .sum::<u32>()
            })
            .sum(),
    )
}

fn main() {
    assert_eq!((157, 70), part12(include_str!("input_test")));

    let (part1, part2) = part12(include_str!("input"));

    println!("Day 03");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
