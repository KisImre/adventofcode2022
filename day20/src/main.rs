// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn part1(input: &str, multiplier: i64, rounds: usize) -> i64 {
    let mut nums: Vec<(usize, i64)> = input
        .split("\n")
        .enumerate()
        .map(|(index, line)| (index, line.parse::<i64>().unwrap() * multiplier))
        .collect();

    let length = nums.len();
    for _round in 0..rounds {
        for i in 0..length {
            let old_index = nums.iter().position(|(index, _value)| *index == i).unwrap();
            let new_index = ((old_index as i64) + nums[old_index].1).rem_euclid(length as i64 - 1);

            let item = nums.remove(old_index);
            nums.insert(new_index as usize, item);
        }
    }

    let zero_pos = nums.iter().position(|a| a.1 == 0).unwrap();

    nums[(zero_pos + 1000) % length].1
        + nums[(zero_pos + 2000) % length].1
        + nums[(zero_pos + 3000) % length].1
}

fn main() {
    let prime = 811589153;

    assert_eq!(3, part1(include_str!("input_test"), 1, 1));
    assert_eq!(1623178306, part1(include_str!("input_test"), prime, 10));

    let p1 = part1(include_str!("input"), 1, 1);
    let p2 = part1(include_str!("input"), prime, 10);

    println!("Day 20");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
