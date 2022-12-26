// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn snafu_to_number(snafu: &str) -> i64 {
    let mut res = 0;
    let mut digit = 1;
    for c in snafu.chars().rev() {
        res += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            c => panic!("MI {}", c),
        } * digit;

        digit *= 5;
    }

    res
}

fn number_to_snafu(mut x: i64) -> String {
    let mut res = Vec::new();
    while x != 0 {
        let rem = x % 5;
        x /= 5;

        let (carry, s) = match rem {
            0 => (0, '0'),
            1 => (0, '1'),
            2 => (0, '2'),
            3 => (1, '='),
            4 => (1, '-'),
            5 => (1, '0'),
            _ => panic!("FOS"),
        };

        x += carry;
        res.push(s)
    }

    res.iter().rev().collect()
}

fn part1(input: &str) -> i64 {
    input.split("\n").map(|line| snafu_to_number(line)).sum()
}

fn main() {
    assert_eq!(1, snafu_to_number("1"));
    assert_eq!(2, snafu_to_number("2"));
    assert_eq!(3, snafu_to_number("1="));
    assert_eq!(4, snafu_to_number("1-"));
    assert_eq!(5, snafu_to_number("10"));
    assert_eq!(6, snafu_to_number("11"));
    assert_eq!(7, snafu_to_number("12"));
    assert_eq!(8, snafu_to_number("2="));
    assert_eq!(9, snafu_to_number("2-"));
    assert_eq!(10, snafu_to_number("20"));
    assert_eq!(15, snafu_to_number("1=0"));
    assert_eq!(20, snafu_to_number("1-0"));
    assert_eq!(2022, snafu_to_number("1=11-2"));
    assert_eq!(12345, snafu_to_number("1-0---0"));
    assert_eq!(314159265, snafu_to_number("1121-1110-1=0"));

    assert_eq!("1=-0-2", number_to_snafu(1747));
    assert_eq!("12111", number_to_snafu(906));
    assert_eq!("2=0=", number_to_snafu(198));
    assert_eq!("21", number_to_snafu(11));
    assert_eq!("2=01", number_to_snafu(201));
    assert_eq!("111", number_to_snafu(31));
    assert_eq!("20012", number_to_snafu(1257));
    assert_eq!("112", number_to_snafu(32));
    assert_eq!("1=-1=", number_to_snafu(353));
    assert_eq!("1-12", number_to_snafu(107));
    assert_eq!("12", number_to_snafu(7));
    assert_eq!("1=", number_to_snafu(3));
    assert_eq!("122", number_to_snafu(37));

    assert_eq!(4890, part1(include_str!("input_test")));

    println!("Day 21");
    println!("Part 1: {}", number_to_snafu(part1(include_str!("input"))));
}
