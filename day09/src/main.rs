// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::HashSet;

fn part12(input: &str, count: usize) -> usize {
    let mut knots: Vec<(i32, i32)> = Vec::new();
    for _i in 0..count {
        knots.push((0, 0));
    }

    let mut tail_places: HashSet<(i32, i32)> = HashSet::new();

    tail_places.insert((0, 0));

    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();

        if line.is_empty() {
            break;
        }

        let dir = parts[0];
        let num: i32 = parts[1].parse().unwrap();

        for _ in 0..num {
            match dir {
                "R" => knots[0].0 += 1,
                "L" => knots[0].0 -= 1,
                "U" => knots[0].1 += 1,
                "D" => knots[0].1 -= 1,
                _ => {}
            }

            for i in 1..knots.len() {
                let prev = knots[i - 1];
                let current = knots[i];

                let diff_x = current.0 - prev.0;
                let diff_y = current.1 - prev.1;

                if diff_x.abs() > 1 || diff_y.abs() > 1 {
                    if diff_x.abs() != 0 && diff_y.abs() != 0 {
                        if diff_x.abs() > 1 && diff_y.abs() > 1 {
                            knots[i] = ((current.0 + prev.0) / 2, (current.1 + prev.1) / 2);
                        } else if diff_x.abs() > 1 {
                            // Move x
                            knots[i] = ((current.0 + prev.0) / 2, prev.1);
                        } else if diff_y.abs() > 1 {
                            // Move y
                            knots[i] = (prev.0, (current.1 + prev.1) / 2);
                        }
                    } else {
                        // Single direction
                        knots[i] = ((current.0 + prev.0) / 2, (current.1 + prev.1) / 2);
                    }

                    if i == (knots.len() - 1) {
                        tail_places.insert(knots[i]);
                    }
                }
            }
        }
    }

    tail_places.len()
}

fn main() {
    assert_eq!(13, part12(include_str!("test_input"), 2));
    assert_eq!(1, part12(include_str!("test_input"), 10));
    assert_eq!(36, part12(include_str!("test_input2"), 10));

    let part1 = part12(include_str!("input"), 2);
    let part2 = part12(include_str!("input"), 10);

    println!("Day 08");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
