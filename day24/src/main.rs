// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::hash::Hash;

use crate::astar::a_star;

mod astar;

#[derive(Clone)]
struct Blizzard {
    position: (usize, usize),
    direction: char,
}

impl Blizzard {
    fn step(&mut self, width: usize, height: usize) {
        match self.direction {
            '>' => {
                if self.position.0 < width - 2 {
                    self.position.0 += 1;
                } else {
                    self.position.0 = 1;
                }
            }
            '<' => {
                if self.position.0 > 1 {
                    self.position.0 -= 1;
                } else {
                    self.position.0 = width - 2;
                }
            }
            'v' => {
                if self.position.1 < height - 2 {
                    self.position.1 += 1;
                } else {
                    self.position.1 = 1;
                }
            }
            '^' => {
                if self.position.1 > 1 {
                    self.position.1 -= 1;
                } else {
                    self.position.1 = height - 2;
                }
            }
            d => panic!("Invalid blizzard direction {}", d),
        }
    }

    fn is_at_position(&self, pos: &Elves) -> bool {
        self.position == (pos.x, pos.y)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Elves {
    x: usize,
    y: usize,
    time: usize,
}

impl Elves {
    fn new(x: usize, y: usize, time: usize) -> Self {
        Self { x, y, time }
    }

    fn manchester_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn is_in_area(&self, width: usize, height: usize) -> bool {
        if 1 <= self.x && self.x < width - 1 && 1 <= self.y && self.y < height - 1 {
            return true;
        }

        if (self.x == 1 && self.y == 0) || (self.x == width - 2 && self.y == height - 1) {
            return true;
        }

        false
    }

    fn get_neighbors(&self) -> Vec<Self> {
        vec![
            Self::new(self.x, self.y, self.time + 1),
            Self::new(self.x + 1, self.y, self.time + 1),
            Self::new(self.x - 1, self.y, self.time + 1),
            Self::new(self.x, self.y + 1, self.time + 1),
            Self::new(self.x, self.y.overflowing_sub(1).0, self.time + 1), // Handle starting point with overflow
        ]
    }
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part12(input: &str, is_part2: bool) -> usize {
    let mut blizzards: Vec<Blizzard> = Vec::new();

    let mut width = 0;
    let mut height = 0;
    for (y, line) in input.split("\n").enumerate() {
        width = line.len();
        height = y + 1;

        for (x, c) in line.chars().enumerate() {
            if c != '#' && c != '.' {
                blizzards.push(Blizzard {
                    position: (x, y),
                    direction: c,
                });
            }
        }
    }

    let size = lcm(width, height);

    let mut blizzard_maps: Vec<Vec<Blizzard>> = Vec::new();
    for _i in 0..size {
        blizzard_maps.push(blizzards.clone());
        for blizzard in blizzards.iter_mut() {
            blizzard.step(width, height);
        }
    }

    let neighbors = |elves: &Elves| {
        let next_blizzards = &blizzard_maps[(elves.time + 1) % size];

        elves
            .get_neighbors()
            .into_iter()
            .filter(|p| {
                p.is_in_area(width, height) && !next_blizzards.iter().any(|b| b.is_at_position(p))
            })
            .collect()
    };
    let distance_func = |a: &Elves, b: &Elves| a.time.abs_diff(b.time);
    let is_goal = |a: &Elves, b: &Elves| a.x == b.x && a.y == b.y;

    let mut result = 0;

    let start = Elves::new(1, 0, 0);
    let goal = Elves::new(width - 2, height - 1, 0);
    let heuristic = |p: &Elves| goal.manchester_distance(p);
    let path1 = a_star(start, goal, heuristic, distance_func, neighbors, is_goal);
    result += path1.len() - 1;

    if is_part2 {
        let start = Elves::new(width - 2, height - 1, result);
        let goal = Elves::new(1, 0, 0);
        let heuristic = |p: &Elves| goal.manchester_distance(p);
        let path2 = a_star(start, goal, heuristic, distance_func, neighbors, is_goal);
        result += path2.len() - 1;

        let start = Elves::new(1, 0, result);
        let goal = Elves::new(width - 2, height - 1, 0);
        let heuristic = |p: &Elves| goal.manchester_distance(p);
        let path3 = a_star(start, goal, heuristic, distance_func, neighbors, is_goal);
        result += path3.len() - 1;
    }

    result
}

fn main() {
    assert_eq!(18, part12(include_str!("input_test"), false));
    assert_eq!(54, part12(include_str!("input_test"), true));

    let p1 = part12(include_str!("input"), false);
    let p2 = part12(include_str!("input"), true);

    println!("Day 24");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
