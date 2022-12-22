// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use core::panic;
use std::collections::HashMap;

use regex::Regex;

struct Map {
    tiles: Vec<Vec<char>>,
    edges: HashMap<(i64, i64, i64), (i64, i64, i64)>,
}

impl Map {
    fn new(tiles: Vec<Vec<char>>, corner_info: Vec<usize>) -> Self {
        let mut edge_points = Vec::new();

        // View from left and right
        for (y, line) in tiles.iter().enumerate() {
            edge_points.push((
                line.iter().position(|c| *c != ' ').unwrap() as i64 - 1,
                y as i64,
                2,
            ));
            edge_points.push((
                line.iter().rposition(|c| *c != ' ').unwrap() as i64 + 1,
                y as i64,
                0,
            ));
        }

        // View from up and down
        for x in 0..(tiles.iter().map(|l| l.len()).max().unwrap()) {
            edge_points.push((
                x as i64,
                tiles
                    .iter()
                    .position(|l| x < l.len() && l[x] != ' ')
                    .unwrap() as i64
                    - 1,
                3,
            ));
            edge_points.push((
                x as i64,
                tiles
                    .iter()
                    .rposition(|l| x < l.len() && l[x] != ' ')
                    .unwrap() as i64
                    + 1,
                1,
            ));
        }

        let mut ordered_edge_points = Vec::new();

        let mut edge_point = edge_points.pop().unwrap();
        ordered_edge_points.push(edge_point);
        while !edge_points.is_empty() {
            if let Some(neighbor) = edge_points.iter().min_by_key(|p| {
                p.0.abs_diff(edge_point.0)
                    + p.1.abs_diff(edge_point.1)
                    + if p.2 == edge_point.2 { 0 } else { 1 }
            }) {
                let neighbor_index = edge_points.iter().position(|p| p == neighbor).unwrap();
                edge_point = edge_points.remove(neighbor_index);
                ordered_edge_points.push(edge_point);
            } else {
                panic!("{:?} {:?}", ordered_edge_points, edge_points);
            }
        }

        let corner_indices = ordered_edge_points
            .iter()
            .zip(ordered_edge_points.iter().cycle().skip(1))
            .enumerate()
            .filter(|(_index, (current, next))| current.0 == next.0 && current.1 == next.1)
            .map(|(index, _)| index)
            .collect::<Vec<_>>();

        let edge_size = ordered_edge_points.len() / 14;

        let mut edges = HashMap::new();

        for (corner, x) in corner_indices.iter().zip(corner_info) {
            let limit = edge_size * x;
            for i in 0..limit {
                let a =
                    ordered_edge_points[(*corner + 1 + i).rem_euclid(ordered_edge_points.len())];
                let b = ordered_edge_points[(*corner - i + ordered_edge_points.len())
                    .rem_euclid(ordered_edge_points.len())];

                edges.insert(a, b);
                edges.insert(b, a);
            }
        }

        Self { tiles, edges }
    }

    fn get_start_tile(&self) -> (i64, i64) {
        (
            self.tiles[0].iter().position(|c| *c == '.').unwrap() as i64,
            0i64,
        )
    }

    fn get_tile(&self, position: &(i64, i64)) -> char {
        *self
            .tiles
            .get(position.1 as usize)
            .unwrap_or(&Vec::new())
            .get(position.0 as usize)
            .unwrap_or(&' ')
    }

    fn get_wrapped_next_position(&self, position: &(i64, i64), direction: i64) -> (i64, i64) {
        match direction {
            0 => (
                self.tiles[position.1 as usize]
                    .iter()
                    .position(|c| *c != ' ')
                    .unwrap() as i64,
                position.1,
            ),
            1 => (
                position.0,
                self.tiles
                    .iter()
                    .position(|l| l[position.0 as usize] != ' ')
                    .unwrap() as i64,
            ),
            2 => (
                self.tiles[position.1 as usize]
                    .iter()
                    .rposition(|c| *c != ' ')
                    .unwrap() as i64,
                position.1,
            ),
            3 => (
                position.0,
                self.tiles
                    .iter()
                    .rposition(|l| l.len() > (position.0 as usize) && l[position.0 as usize] != ' ')
                    .unwrap() as i64,
            ),
            dir => panic!("Invalid direction {}", dir),
        }
    }

    fn get_3d_wrapped_next_position(
        &self,
        next_pos: &(i64, i64),
        direction: i64,
    ) -> ((i64, i64), i64) {
        let next_pos_3d = self
            .edges
            .get(&(next_pos.0, next_pos.1, direction))
            .unwrap();

        let dir = (next_pos_3d.2 + 2).rem_euclid(4);

        let res = match dir {
            0 => (next_pos_3d.0 + 1, next_pos_3d.1),
            1 => (next_pos_3d.0, next_pos_3d.1 + 1),
            2 => (next_pos_3d.0 - 1, next_pos_3d.1),
            3 => (next_pos_3d.0, next_pos_3d.1 - 1),
            dir => panic!("Invalid direction {}", dir),
        };

        ((res.0, res.1), dir)
    }
}

fn parse_input(input: &str, corner_info: Vec<usize>) -> (Map, Vec<(i64, char)>) {
    let mut is_map = true;
    let mut map = Vec::new();
    let mut path_string = String::new();

    for line in input.split("\n") {
        if line.is_empty() {
            is_map = false;
            continue;
        }

        if is_map {
            map.push(line.chars().collect());
        } else {
            path_string = String::from(line);
        }
    }

    let pattern = Regex::new("([0-9]+)([RL]?)").unwrap();

    let path = pattern
        .captures_iter(path_string.as_str())
        .map(|m| (m[1].parse().unwrap(), m[2].chars().next().unwrap_or(' ')))
        .collect();

    (Map::new(map, corner_info), path)
}

fn part12(input: &str, is_part2: bool, corner_info: Vec<usize>) -> usize {
    let (map, path) = parse_input(input, corner_info);

    let mut position = map.get_start_tile();
    let mut direction = 0i64;

    for (steps, turn) in path {
        for _step in 0..steps {
            let mut next_pos = match direction {
                0 => (position.0 + 1, position.1),
                1 => (position.0, position.1 + 1),
                2 => (position.0 - 1, position.1),
                3 => (position.0, position.1 - 1),
                dir => panic!("Invalid direction {}", dir),
            };

            let mut next_tile = map.get_tile(&next_pos);
            if next_tile == ' ' {
                // Do the wrapping
                if !is_part2 {
                    next_pos = map.get_wrapped_next_position(&position, direction);
                } else {
                    let (next_pos_3d, direction_3d) =
                        map.get_3d_wrapped_next_position(&next_pos, direction);
                    next_pos = next_pos_3d;
                    if map.get_tile(&next_pos) == '.' {
                        // Only update direction if it's not an obstacle on the 3D next tile
                        direction = direction_3d;
                    }
                }
                next_tile = map.get_tile(&next_pos);
            }
            position = match next_tile {
                '.' => next_pos,
                '#' => position,
                t => panic!("Invalid tile {}", t),
            };
        }

        direction = match turn {
            'L' => (direction - 1).rem_euclid(4),
            'R' => (direction + 1).rem_euclid(4),
            ' ' => direction,
            t => panic!("Invalid turn {}", t),
        };
    }

    ((position.1 + 1) * 1000 + (position.0 + 1) * 4 + direction as i64) as usize
}

fn main() {
    assert_eq!(
        6032,
        part12(include_str!("input_test"), false, vec![3, 2, 2])
    );
    assert_eq!(
        5031,
        part12(include_str!("input_test"), true, vec![3, 2, 2])
    );

    let part1 = part12(include_str!("input"), false, vec![2, 1, 4]);
    let part2 = part12(include_str!("input"), true, vec![2, 1, 4]);

    println!("Day 22");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
