// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::BTreeSet;

#[derive(PartialEq, Eq)]
enum Shape {
    Minus,
    Plus,
    L,
    I,
    Dot,
}

impl Shape {
    fn next(&self) -> Self {
        match self {
            Shape::Minus => Self::Plus,
            Shape::Plus => Self::L,
            Shape::L => Self::I,
            Shape::I => Self::Dot,
            Shape::Dot => Self::Minus,
        }
    }
}

#[derive(Debug)]
struct Rock {
    parts: Vec<(u64, u64)>,
}

impl Rock {
    fn new(base: u64, shape: &Shape) -> Self {
        let bottom = base + 4;
        Self {
            parts: match shape {
                Shape::Minus => vec![(2, bottom), (3, bottom), (4, bottom), (5, bottom)],
                Shape::Plus => vec![
                    (2, bottom + 1),
                    (3, bottom + 1),
                    (4, bottom + 1),
                    (3, bottom),
                    (3, bottom + 2),
                ],
                Shape::L => vec![
                    (2, bottom),
                    (3, bottom),
                    (4, bottom),
                    (4, bottom + 1),
                    (4, bottom + 2),
                ],
                Shape::I => vec![
                    (2, bottom),
                    (2, bottom + 1),
                    (2, bottom + 2),
                    (2, bottom + 3),
                ],
                Shape::Dot => vec![(2, bottom), (3, bottom), (2, bottom + 1), (3, bottom + 1)],
            },
        }
    }

    fn move_sideways(&mut self, direction: i64, used: &BTreeSet<(u64, u64)>) {
        let limit = if direction < 0 { 0 } else { 6 };
        if self
            .parts
            .iter()
            .all(|p| p.0 != limit && !used.contains(&((p.0 as i64 + direction) as u64, p.1)))
        {
            for part in &mut self.parts {
                if direction < 0 {
                    part.0 -= direction.abs() as u64;
                } else {
                    part.0 += direction.abs() as u64;
                }
            }
        }
    }

    fn can_fall(&self, used: &BTreeSet<(u64, u64)>) -> bool {
        self.parts
            .iter()
            .all(|part| part.1 > 0 && !used.contains(&(part.0, part.1 - 1)))
    }

    fn fall(&mut self) {
        for part in &mut self.parts {
            part.1 -= 1;
        }
    }

    fn update_peaks_and_used(&self, peaks: &mut Vec<u64>, used: &mut BTreeSet<(u64, u64)>) {
        for part in &self.parts {
            peaks[part.0 as usize] = peaks[part.0 as usize].max(part.1);
            used.insert(*part);
        }
    }
}

fn part1(input: &str, limit: usize) -> (u64, Vec<(usize, u64)>) {
    let mut directions = input.chars().cycle();

    let mut peaks: Vec<u64> = vec![0; 7];
    let mut shape = Shape::Minus;

    let mut used: BTreeSet<(u64, u64)> = BTreeSet::new();
    let mut flats: Vec<(usize, u64)> = Vec::new();

    for x in 0..7 {
        used.insert((x, 0));
    }

    for rock_count in 0..limit {
        let base = *peaks.iter().max_by(|a, b| a.cmp(b)).unwrap();
        let mut rock = Rock::new(base, &shape);

        loop {
            let c = directions.next().unwrap();
            let direction = match c {
                '<' => -1,
                '>' => 1,
                _ => panic!("Invalid char {}", c),
            };
            rock.move_sideways(direction, &used);

            if rock.can_fall(&used) {
                rock.fall();
            } else {
                break;
            }
        }
        rock.update_peaks_and_used(&mut peaks, &mut used);
        if peaks.iter().all(|a| *a == peaks[0]) {
            flats.push((rock_count, peaks[0]))
        }
        shape = shape.next();
    }

    (*peaks.iter().max_by(|a, b| a.cmp(b)).unwrap(), flats)
}

fn part2(input: &str, count: usize) -> u64 {
    let (_, flats) = part1(input, 3000);

    let first_flat = flats[0].0;
    let flat_period = flats[1].0 - flats[0].0;
    let period_height = flats[1].1 - flats[0].1;

    let full_periods: u64 = ((count - first_flat) / flat_period) as u64 * period_height;
    let remainder = part1(input, ((count - first_flat) % flat_period) + first_flat).0;

    full_periods + remainder
}

fn main() {
    assert_eq!(3068, part1(include_str!("input_test"), 2022).0);

    let p1 = part1(include_str!("input"), 2022);
    let p2 = part2(include_str!("input"), 1000000000000);

    println!("Day 17");
    println!("Part 1: {}", p1.0);
    println!("Part 2: {}", p2);
}
