// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

use regex::Regex;

struct Sensor {
    position: (i64, i64),
    beacon: (i64, i64),
}

impl Sensor {
    fn new(position: (i64, i64), beacon: (i64, i64)) -> Self {
        Self { position, beacon }
    }

    fn get_radius(&self) -> i64 {
        Self::manchester_distance(&self.position, &self.beacon)
    }

    fn in_radius(&self, position: &(i64, i64)) -> bool {
        Self::manchester_distance(&self.position, &position) <= self.get_radius()
    }

    fn get_covered_position_in_line(&self, observed_line: i64) -> Vec<(i64, i64)> {
        let radius = self.get_radius();
        ((self.position.0 - radius)..=(self.position.0 + radius))
            .map(|x| (x, observed_line))
            .filter(|i| self.in_radius(&i))
            .collect()
    }

    fn get_outer_edges(&self, area_max: i64) -> Vec<(i64, i64)> {
        let radius = self.get_radius() + 1;

        (0..radius)
            .map(|i| {
                [
                    (self.position.0 + radius - i, self.position.1 + i),
                    (self.position.0 - radius + i, self.position.1 + i),
                    (self.position.0 + radius - i, self.position.1 - i),
                    (self.position.0 - radius + i, self.position.1 - i),
                ]
            })
            .flatten()
            .filter(|item| 0 <= item.0 && item.0 <= area_max && 0 <= item.1 && item.1 <= area_max)
            .collect()
    }

    fn manchester_distance(a: &(i64, i64), b: &(i64, i64)) -> i64 {
        (a.0 - b.0).abs() + (a.1 - b.1).abs()
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let pattern =
        Regex::new(r"Sensor at x=([^,]+), y=([^:]+): closest beacon is at x=([^,]+), y=(.+)")
            .unwrap();

    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            let matches = pattern.captures(line).unwrap();

            Sensor::new(
                (matches[1].parse().unwrap(), matches[2].parse().unwrap()),
                (matches[3].parse().unwrap(), matches[4].parse().unwrap()),
            )
        })
        .collect()
}

fn part1(input: &str, observed_line: i64) -> usize {
    let sensors = parse_input(input);

    let mut covered_positions = BTreeSet::new();
    for sensor in sensors {
        for position in sensor.get_covered_position_in_line(observed_line) {
            covered_positions.insert(position.0);
        }
    }

    covered_positions.len() - 1
}

fn part2(input: &str, area_max: i64) -> u64 {
    let sensors = parse_input(input);

    for sensor in &sensors {
        for candidate in sensor.get_outer_edges(area_max) {
            if sensors.iter().all(|s| !s.in_radius(&candidate)) {
                return candidate.0 as u64 * 4000000 + candidate.1 as u64;
            }
        }
    }

    panic!("Beacon not found");
}

fn main() {
    assert_eq!(26, part1(include_str!("input_test"), 10));
    assert_eq!(56000011, part2(include_str!("input_test"), 20));

    let part1 = part1(include_str!("input"), 2000000);
    let part2 = part2(include_str!("input"), 4000000);

    println!("Day 14");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
