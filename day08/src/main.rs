// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn part1(input: &str) -> usize {
    let mut heights: Vec<Vec<u8>> = Vec::new();
    let mut visible: Vec<Vec<bool>> = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        heights.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
        visible.push(vec![false; line.len()]);
    }

    for row in 0..heights.len() {
        let mut m = heights[row][0];
        visible[row][0] = true;
        for col in 1..heights[row].len() {
            if heights[row][col] > m {
                m = heights[row][col];
                visible[row][col] = true;
            }
        }
    }

    for row in 0..heights.len() {
        let mut m = heights[row][heights[row].len() - 1];
        visible[row][heights[row].len() - 1] = true;
        for col in (0..heights[row].len()).rev() {
            if heights[row][col] > m {
                m = heights[row][col];
                visible[row][col] = true;
            }
        }
    }

    for col in 0..heights[0].len() {
        let mut m = heights[0][col];
        visible[0][col] = true;
        for row in 1..heights.len() {
            if heights[row][col] > m {
                m = heights[row][col];
                visible[row][col] = true;
            }
        }
    }

    for col in 0..heights[0].len() {
        let mut m = heights[heights.len() - 1][col];
        visible[heights.len() - 1][col] = true;
        for row in (0..heights.len()).rev() {
            if heights[row][col] > m {
                m = heights[row][col];
                visible[row][col] = true;
            }
        }
    }

    visible.into_iter().flatten().filter(|i| *i).count()
}

fn part2(input: &str) -> usize {
    let mut heights: Vec<Vec<u8>> = Vec::new();
    let mut scenic_scores: Vec<Vec<u32>> = Vec::new();

    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }
        heights.push(
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect(),
        );
        scenic_scores.push(vec![0; line.len()]);
    }

    for row in 0..heights.len() {
        for col in 0..heights[row].len() {
            let mut score = 1;

            // Up
            let mut count = 0;
            for i in (0..row).rev() {
                count += 1;
                if heights[i][col] >= heights[row][col] {
                    break;
                }
            }
            score *= count;

            // Down
            let mut count = 0;
            for i in row + 1..heights.len() {
                count += 1;
                if heights[i][col] >= heights[row][col] {
                    break;
                }
            }
            score *= count;

            // Left
            let mut count = 0;
            for i in (0..col).rev() {
                count += 1;
                if heights[row][i] >= heights[row][col] {
                    break;
                }
            }
            score *= count;

            // Right
            let mut count = 0;
            for i in col + 1..heights[row].len() {
                count += 1;
                if heights[row][i] >= heights[row][col] {
                    break;
                }
            }
            score *= count;

            scenic_scores[row][col] = score;
        }
    }

    scenic_scores.into_iter().flatten().max().unwrap() as usize
}

fn main() {
    assert_eq!(21, part1(include_str!("input_test")));
    assert_eq!(8, part2(include_str!("input_test")));

    let part1 = part1(include_str!("input"));
    let part2 = part2(include_str!("input"));

    println!("Day 08");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
