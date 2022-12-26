// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::{BTreeMap, BTreeSet};

fn part12(input: &str) -> (usize, usize) {
    let mut elves = BTreeSet::new();

    for (y, line) in input.split("\n").enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    let offsets = [
        [(-1, -1), (0, -1), (1, -1)],
        [(-1, 1), (0, 1), (1, 1)],
        [(-1, -1), (-1, 0), (-1, 1)],
        [(1, -1), (1, 0), (1, 1)],
    ];

    let mut round_10_res = 0;
    let mut round = 0;
    loop {
        if round == 10 {
            let min_x = *elves.iter().map(|(x, _y)| x).min().unwrap();
            let max_x = *elves.iter().map(|(x, _y)| x).max().unwrap();
            let min_y = *elves.iter().map(|(_x, y)| y).min().unwrap();
            let max_y = *elves.iter().map(|(_x, y)| y).max().unwrap();

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    if !elves.contains(&(x, y)) {
                        round_10_res += 1;
                    }
                }
            }
        }

        let mut new_positions = BTreeMap::new();

        // Check new positions
        for elf in &elves {
            let no_need_to_move = offsets
                .iter()
                .flatten()
                .all(|d| !elves.contains(&(elf.0 + d.0, elf.1 + d.1)));
            if no_need_to_move {
                continue;
            }

            for dir in offsets.iter().cycle().skip(round).take(4) {
                if dir
                    .iter()
                    .all(|d| !elves.contains(&(elf.0 + d.0, elf.1 + d.1)))
                {
                    new_positions.insert(elf, (elf.0 + dir[1].0, elf.1 + dir[1].1));
                    break;
                }
            }
        }

        if new_positions.is_empty() {
            break;
        }

        // Move elves
        let mut new_elves = BTreeSet::new();
        for elf in &elves {
            if let Some(new_pos) = new_positions.get(&elf) {
                if new_positions.iter().filter(|p| *p.1 == *new_pos).count() <= 1 {
                    // Able to move
                    new_elves.insert(*new_pos);
                } else {
                    // Conflict in move
                    new_elves.insert(*elf);
                }
            } else {
                // Couldn't move at the beginning
                new_elves.insert(*elf);
            }
        }

        elves = new_elves;

        round += 1;
    }

    (round_10_res, round + 1)
}

fn main() {
    assert_eq!((110, 20), part12(include_str!("input_test")));

    let p12 = part12(include_str!("input"));
    println!("Day 23");
    println!("Part 1: {:?}", p12.0);
    println!("Part 2: {:?}", p12.1);
}
