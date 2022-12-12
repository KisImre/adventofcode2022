// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn part12(input: &str, reverse: bool) -> String {
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); 9];

    let mut init = true;
    for line in input.split("\n") {
        if line.is_empty() {
            init = false;
            continue;
        }

        if init {
            for (i, c) in line
                .chars()
                .enumerate()
                .skip(1)
                .step_by(4)
                .filter(|(_i, c)| !c.is_numeric() && !c.is_whitespace())
            {
                stacks[i / 4].insert(0, c);
            }
        } else {
            let line_parts: Vec<&str> = line.split(" ").collect();
            let count: usize = line_parts[1].parse().unwrap();
            let from: usize = line_parts[3].parse().unwrap();
            let to: usize = line_parts[5].parse().unwrap();

            let mut stack_to_move: Vec<_> = (0..count)
                .map(|_| stacks[from - 1].pop().unwrap())
                .collect();

            if reverse {
                stack_to_move.reverse();
            }

            stacks[to - 1].append(&mut stack_to_move);
        }
    }

    stacks
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s[s.len() - 1])
        .collect::<String>()
}

fn main() {
    assert_eq!("CMZ", part12(include_str!("input_test"), false));
    assert_eq!("MCD", part12(include_str!("input_test"), true));

    let part1 = part12(include_str!("input"), false);
    let part2 = part12(include_str!("input"), true);

    println!("Day 05");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
