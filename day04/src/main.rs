// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn part12(input: &str) -> (u32, u32) {
    input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect();

            let range0: Vec<&str> = parts[0].split("-").collect();
            let range1: Vec<&str> = parts[1].split("-").collect();

            let range0_start: u32 = range0[0].parse().unwrap();
            let range0_end: u32 = range0[1].parse().unwrap();
            let range1_start: u32 = range1[0].parse().unwrap();
            let range1_end: u32 = range1[1].parse().unwrap();

            (
                ((range0_start <= range1_start && range1_end <= range0_end)
                    || (range1_start <= range0_start && range0_end <= range1_end))
                    as u32,
                ((range0_start <= range1_start && range1_start <= range0_end)
                    || (range1_start <= range0_start && range0_start <= range1_end))
                    as u32,
            )
        })
        .reduce(|acc, i| (acc.0 + i.0, acc.1 + i.1))
        .unwrap()
}

fn main() {
    assert_eq!((2, 4), part12(include_str!("input_test")));

    let (part1, part2) = part12(include_str!("input"));

    println!("Day 04");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
