// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

#![feature(iter_array_chunks)]

use std::cmp::Ordering;

enum Item {
    Int(u32),
    List(Vec<Item>),
}

impl Item {
    fn from_str(input: &str) -> Self {
        let list = Self::split(input);

        Item::List(
            list.iter()
                .map(|str_item| {
                    if let Ok(integer) = str_item.parse() {
                        Item::Int(integer)
                    } else {
                        Self::from_str(str_item)
                    }
                })
                .collect(),
        )
    }

    fn split(input: &str) -> Vec<String> {
        let mut res = Vec::new();

        let mut temp = String::new();
        let mut level = 0;
        for c in input.chars() {
            match c {
                '[' => {
                    if level > 0 {
                        temp.push(c);
                    }
                    level += 1;
                }
                ']' => {
                    level -= 1;
                    if level > 0 {
                        temp.push(c);
                    }
                }
                ',' => {
                    if level == 1 {
                        res.push(temp.clone());
                        temp.clear();
                    } else {
                        temp.push(c);
                    }
                }
                c => temp.push(c),
            }
        }
        if !temp.is_empty() {
            res.push(temp);
        }

        res
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Item::Int(a), Item::Int(b)) => a.cmp(b),
            (Item::Int(i1), l2) => Item::List(vec![Item::Int(*i1)]).cmp(l2),
            (l1, Item::Int(i2)) => l1.cmp(&Item::List(vec![Item::Int(*i2)])),
            (Item::List(l1), Item::List(l2)) => {
                for (i1, i2) in l1.iter().zip(l2.iter()) {
                    let res = i1.cmp(i2);

                    if res != Ordering::Equal {
                        return res;
                    }
                }

                l1.len().cmp(&l2.len())
            }
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        Self::cmp(&self, other) == Ordering::Equal
    }
}

impl Eq for Item {}

fn part1(input: &str) -> usize {
    let mut count = 0;

    for (index, [line1, line2, _]) in input.split("\n").array_chunks().enumerate() {
        if Item::from_str(line1) < Item::from_str(line2) {
            count += index + 1;
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut items: Vec<_> = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|l| Item::from_str(l))
        .collect();

    items.push(Item::from_str("[[2]]"));
    items.push(Item::from_str("[[6]]"));

    items.sort();

    let a = items
        .iter()
        .position(|i| *i == Item::from_str("[[2]]"))
        .unwrap();
    let b = items
        .iter()
        .position(|i| *i == Item::from_str("[[6]]"))
        .unwrap();

    (a + 1) * (b + 1)
}

fn main() {
    assert_eq!(13, part1(include_str!("input_test")));
    assert_eq!(140, part2(include_str!("input_test")));

    let part1 = part1(include_str!("input"));
    let part2 = part2(include_str!("input"));
    println!("Day 13");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
