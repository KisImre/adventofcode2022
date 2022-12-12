// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, usize>,
}

impl Dir {
    pub fn new() -> Self {
        Self {
            dirs: HashMap::new(),
            files: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, path: &[String], name: String, len: usize) {
        if path.is_empty() {
            self.files.insert(name.clone(), len);
            return;
        }

        if !self.dirs.contains_key(&path[0]) {
            self.dirs.insert(path[0].clone(), Dir::new());
        }

        self.dirs
            .get_mut(&path[0])
            .unwrap()
            .add_file(&path[1..], name, len);
    }

    pub fn get_directory_size(&self) -> usize {
        self.dirs
            .iter()
            .fold(0, |s, d| s + d.1.get_directory_size())
            + self.files.iter().fold(0, |s, d| s + d.1)
    }

    pub fn get_directory_size_if_larger(&self, required_min_size: usize) -> usize {
        let mut sum = self.get_directory_size();

        if sum > required_min_size {
            sum = 0;
        }

        sum + self
            .dirs
            .iter()
            .map(|d| d.1.get_directory_size_if_larger(required_min_size))
            .sum::<usize>()
    }

    pub fn find_minimal_largers(&self, required_min_size: usize, current_minimum: usize) -> usize {
        let sum = self.get_directory_size();
        let mut minimum = if sum >= required_min_size && sum < current_minimum {
            sum
        } else {
            current_minimum
        };

        for d in &self.dirs {
            minimum = d.1.find_minimal_largers(required_min_size, minimum);
        }

        minimum
    }
}

fn part12(input: &str) -> (usize, usize) {
    let mut tree = Dir::new();

    let mut current_dir = Vec::new();
    for line in input.split("\n") {
        if line.starts_with("$ cd") {
            let dir = line.replace("$ cd ", "");

            if dir == ".." {
                current_dir.pop();
            } else {
                current_dir.push(dir);
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") || line.is_empty() {
            // none
        } else {
            // File
            let parts: Vec<&str> = line.split_whitespace().collect();
            let len: usize = parts[0].parse().unwrap();
            tree.add_file(&current_dir, String::from(parts[1]), len);
        }
    }

    let required_min_size = tree.get_directory_size() - (70000000 - 30000000);

    (
        tree.get_directory_size_if_larger(100000),
        tree.find_minimal_largers(required_min_size, tree.get_directory_size()),
    )
}

fn main() {
    assert_eq!((95437, 24933642), part12(include_str!("input_test")));

    let (part1, part2) = part12(include_str!("input"));

    println!("Day 07");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
