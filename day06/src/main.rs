// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn part12(input: &str, window: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();

    chars
        .windows(window)
        .enumerate()
        .filter(|(_i, w)| w.iter().all(|c| w.iter().filter(|a| *a == c).count() == 1))
        .map(|i| i.0 + window)
        .next()
        .unwrap()
}

fn main() {
    assert_eq!(7, part12("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(5, part12("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, part12("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, part12("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, part12("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));

    assert_eq!(19, part12("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, part12("bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, part12("nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, part12("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, part12("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));

    let part1 = part12(include_str!("input"), 4);
    let part2 = part12(include_str!("input"), 14);

    println!("Day 06");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
