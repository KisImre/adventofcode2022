// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::{cmp::max, cmp::min, collections::BTreeSet};

fn parse_input(input: &str) -> BTreeSet<(i32, i32)> {
    let mut tiles = BTreeSet::new();

    for line in input.split("\n") {
        let rock_paths: Vec<&str> = line.split(" -> ").collect();
        for (path_start, path_end) in rock_paths.iter().zip(rock_paths.iter().skip(1)) {
            let start = path_start
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let end = path_end
                .split(",")
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>();

            if start[0] == end[0] {
                for y in min(start[1], end[1])..=max(start[1], end[1]) {
                    tiles.insert((start[0], y));
                }
            } else {
                for x in min(start[0], end[0])..=max(start[0], end[0]) {
                    tiles.insert((x, start[1]));
                }
            }
        }
    }

    tiles
}

fn part1(input: &str) -> usize {
    let mut tiles = parse_input(input);
    let start_of_abyss = *tiles.iter().map(|(_x, y)| y).max().unwrap();

    let mut sand_count = 0;
    let mut not_falling_to_abyss = true;
    while not_falling_to_abyss {
        let mut sand = (500, 0);
        let mut is_falling = true;
        while is_falling {
            is_falling = false;

            for offset in [(0, 1), (-1, 1), (1, 1)] {
                let new_pos = (sand.0 + offset.0, sand.1 + offset.1);
                if !tiles.contains(&new_pos) {
                    sand = new_pos;
                    is_falling = true;
                    break;
                }
            }

            if is_falling && sand.1 > start_of_abyss {
                not_falling_to_abyss = false;
                break;
            }
        }
        tiles.insert(sand);

        sand_count += 1;
    }

    sand_count - 1
}

fn part2(input: &str) -> usize {
    let mut tiles = parse_input(input);

    let start_of_abyss = *tiles.iter().map(|(_x, y)| y).max().unwrap();

    let mut sand_count = 0;
    loop {
        let mut sand = (500, 0);

        if tiles.contains(&sand) {
            break;
        }

        let mut is_falling = true;
        while is_falling {
            is_falling = false;

            for offset in [(0, 1), (-1, 1), (1, 1)] {
                let new_pos = (sand.0 + offset.0, sand.1 + offset.1);
                if !tiles.contains(&new_pos) {
                    sand = new_pos;
                    is_falling = true;
                    break;
                }
            }

            if sand.1 > start_of_abyss {
                break;
            }
        }
        tiles.insert(sand);

        sand_count += 1;
    }

    sand_count
}

fn main() {
    assert_eq!(24, part1(include_str!("input_test")));
    assert_eq!(93, part2(include_str!("input_test")));

    let part1 = part1(include_str!("input"));
    let part2 = part2(include_str!("input"));

    println!("Day 14");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
