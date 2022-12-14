// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

fn part12(input: &str) -> (u32, u32) {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let mut lowest_points = Vec::new();

    let heights: Vec<Vec<u8>> = input
        .split("\n")
        .enumerate()
        .map(|(row_index, line)| {
            line.chars()
                .enumerate()
                .map(|(col_index, c)| {
                    let height = if c == 'S' {
                        start = (row_index, col_index);
                        0
                    } else if c == 'E' {
                        end = (row_index, col_index);
                        25
                    } else {
                        c.to_digit(36).unwrap() as u8 - 10
                    };

                    if height == 0 {
                        lowest_points.push((row_index, col_index));
                    }

                    height
                })
                .collect()
        })
        .collect();

    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
    let mut distances: Vec<Vec<u32>> = heights
        .iter()
        .map(|row| vec![u32::MAX - 1000; row.len()])
        .collect();
    let mut q: BTreeSet<(usize, usize)> = BTreeSet::new();

    for row in 0..heights.len() {
        for col in 0..heights[0].len() {
            q.insert((row, col));
        }
    }

    distances[end.0][end.1] = 0;

    let neighbour_offsets = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    loop {
        let min = q
            .iter()
            .min_by(|a, b| distances[a.0][a.1].cmp(&distances[b.0][b.1]));

        if let Some(min_pos) = min {
            let u = min_pos.clone();
            q.remove(&u);

            for v in neighbour_offsets
                .iter()
                .map(|(r, c)| ((r + u.0 as i32) as usize, (c + u.1 as i32) as usize))
                .filter(|v| q.contains(&v))
            {
                let current_height = heights[u.0][u.1];
                let neighbour_height = heights[v.0][v.1];

                if current_height <= neighbour_height + 1 {
                    let alt = distances[u.0][u.1] + 1;
                    if alt < distances[v.0][v.1] {
                        distances[v.0][v.1] = alt;
                    }
                }
            }
        } else {
            break;
        }
    }

    (
        distances[start.0][start.1],
        lowest_points
            .iter()
            .map(|p| distances[p.0][p.1])
            .min()
            .unwrap(),
    )
}

fn main() {
    assert_eq!((31, 29), part12(include_str!("input_test")));

    let (part1, part2) = part12(include_str!("input"));

    println!("Day 12");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
