// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

fn parse_input(input: &str) -> BTreeSet<(i64, i64, i64)> {
    input
        .split("\n")
        .map(|line| {
            let parts: Vec<i64> = line.split(",").map(|a| a.parse().unwrap()).collect();
            (parts[0], parts[1], parts[2])
        })
        .collect()
}

const OFFSETS: [(i64, i64, i64); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];

fn get_outside_air_blocks(blocks: &BTreeSet<(i64, i64, i64)>) -> BTreeSet<(i64, i64, i64)> {
    let min = (
        blocks.iter().map(|a| a.0).min().unwrap() - 1,
        blocks.iter().map(|a| a.1).min().unwrap() - 1,
        blocks.iter().map(|a| a.2).min().unwrap() - 1,
    );
    let max = (
        blocks.iter().map(|a| a.0).max().unwrap() + 1,
        blocks.iter().map(|a| a.1).max().unwrap() + 1,
        blocks.iter().map(|a| a.2).max().unwrap() + 1,
    );

    // Flood fill
    let mut air: BTreeSet<(i64, i64, i64)> = BTreeSet::new();
    air.insert(min);
    loop {
        let mut new_air: BTreeSet<(i64, i64, i64)> = BTreeSet::new();

        for a in air.iter() {
            for offset in OFFSETS {
                let p = (a.0 + offset.0, a.1 + offset.1, a.2 + offset.2);

                if min.0 <= p.0
                    && p.0 <= max.0
                    && min.1 <= p.1
                    && p.1 <= max.1
                    && min.2 <= p.2
                    && p.2 <= max.2
                    && !air.contains(&p)
                    && !blocks.contains(&p)
                {
                    new_air.insert(p);
                }
            }
        }

        if new_air.is_empty() {
            break;
        } else {
            air.append(&mut new_air);
        }
    }
    air
}

fn part1(input: &str) -> u64 {
    let blocks = parse_input(input);
    let mut surface = 0;
    for block in &blocks {
        for offset in OFFSETS {
            let neighbour = (block.0 + offset.0, block.1 + offset.1, block.2 + offset.2);
            if !blocks.contains(&neighbour) {
                surface += 1;
            }
        }
    }

    surface
}

fn part2(input: &str) -> u64 {
    let blocks = parse_input(input);
    let outside_air = get_outside_air_blocks(&blocks);

    let mut surface = 0;
    for block in &blocks {
        for offset in OFFSETS {
            let neighbour = (block.0 + offset.0, block.1 + offset.1, block.2 + offset.2);
            if outside_air.contains(&neighbour) {
                surface += 1;
            }
        }
    }

    surface
}

fn main() {
    assert_eq!(64, part1(include_str!("input_test")));
    assert_eq!(58, part2(include_str!("input_test")));

    let part1 = part1(include_str!("input"));
    let part2 = part2(include_str!("input"));

    println!("Day 18");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
