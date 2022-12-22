// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::{collections::HashMap, hash::Hash};

use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash)]
struct State {
    minutes: u16,

    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,

    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
}

impl State {
    fn new(minutes: u16) -> Self {
        Self {
            minutes,
            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }

    fn mine_minerals(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }
}

struct Blueprint {
    id: u16,
    ore_robot_ore_cost: u16,
    clay_robot_ore_cost: u16,
    obsidian_robot_ore_cost: u16,
    obsidian_robot_clay_cost: u16,
    geode_robot_ore_cost: u16,
    geode_robot_obsidian_cost: u16,

    max_ore_cost: u16,
}

impl Blueprint {
    fn new(line: &str) -> Self {
        let pattern = Regex::new("Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore. Each clay robot costs ([0-9]+) ore. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian.").unwrap();
        let parts = pattern.captures(line).unwrap();

        let ore_robot_ore_cost = parts[2].parse().unwrap();
        let clay_robot_ore_cost = parts[3].parse().unwrap();
        let obsidian_robot_ore_cost = parts[4].parse().unwrap();
        let geode_robot_ore_cost = parts[6].parse().unwrap();

        Self {
            id: parts[1].parse().unwrap(),
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost: parts[5].parse().unwrap(),
            geode_robot_ore_cost,
            geode_robot_obsidian_cost: parts[7].parse().unwrap(),

            max_ore_cost: *[
                ore_robot_ore_cost,
                clay_robot_ore_cost,
                obsidian_robot_ore_cost,
                geode_robot_ore_cost,
            ]
            .iter()
            .max()
            .unwrap(),
        }
    }

    fn step(
        &self,
        mut state: State,
        cache: &mut HashMap<State, u16>,
        current_max: &mut u16,
    ) -> u16 {
        if state.minutes == 0 {
            if state.geode > *current_max {
                *current_max = state.geode;
            }
            return state.geode;
        }

        let best_possibles_geode_amount = state.minutes * (state.minutes - 1) / 2
            + state.geode_robots * state.minutes
            + state.geode;

        if *current_max > best_possibles_geode_amount {
            return 0;
        }

        if let Some(cached_value) = cache.get(&state) {
            return *cached_value;
        }

        state.minutes -= 1;

        let can_build_geode_robot = state.ore >= self.geode_robot_ore_cost
            && state.obsidian >= self.geode_robot_obsidian_cost;
        let can_build_obsidian_robot = state.ore >= self.obsidian_robot_ore_cost
            && state.clay >= self.obsidian_robot_clay_cost
            && state.obsidian_robots < self.geode_robot_obsidian_cost;
        let can_build_clay_robot = state.ore >= self.clay_robot_ore_cost
            && state.clay_robots < self.obsidian_robot_clay_cost;
        let can_build_ore_robot =
            state.ore >= self.ore_robot_ore_cost && state.ore_robots < self.max_ore_cost;

        state.mine_minerals();

        // Not building anything
        let mut max_geodes = self.step(state.clone(), cache, current_max);

        if can_build_geode_robot {
            // Build geode robot
            let mut state = state.clone();
            state.geode_robots += 1;
            state.ore -= self.geode_robot_ore_cost;
            state.obsidian -= self.geode_robot_obsidian_cost;
            max_geodes = max_geodes.max(self.step(state, cache, current_max));
        }

        if can_build_obsidian_robot {
            // Build obsidian robot
            let mut state = state.clone();
            state.obsidian_robots += 1;
            state.ore -= self.obsidian_robot_ore_cost;
            state.clay -= self.obsidian_robot_clay_cost;
            max_geodes = max_geodes.max(self.step(state, cache, current_max));
        }

        if can_build_clay_robot {
            // Build clay robot
            let mut state = state.clone();
            state.clay_robots += 1;
            state.ore -= self.clay_robot_ore_cost;
            max_geodes = max_geodes.max(self.step(state, cache, current_max));
        }

        if can_build_ore_robot {
            // Build ore robot
            let mut state = state.clone();
            state.ore_robots += 1;
            state.ore -= self.ore_robot_ore_cost;
            max_geodes = max_geodes.max(self.step(state, cache, current_max));
        }

        cache.insert(state, max_geodes);

        max_geodes
    }

    fn get_max_geodes(&self, minutes: u16) -> u16 {
        let mut cache: HashMap<State, u16> = HashMap::new();
        let state = State::new(minutes);
        let mut current_max = 0;
        self.step(state, &mut cache, &mut current_max)
    }

    fn get_quality_level(&self, minutes: u16) -> u16 {
        self.id * self.get_max_geodes(minutes)
    }
}

fn part1(input: &str) -> u64 {
    input
        .split("\n")
        .map(|line| Blueprint::new(line).get_quality_level(24) as u64)
        .sum()
}

fn part2(input: &str) -> Vec<u64> {
    input
        .split("\n")
        .take(3)
        .map(|line| Blueprint::new(line).get_max_geodes(32) as u64)
        .collect()
}

fn main() {
    assert_eq!(33, part1(include_str!("input_test")));
    assert_eq!(62, *part2(include_str!("input_test")).iter().max().unwrap());

    let p1 = part1(include_str!("input"));
    let p2: u64 = part2(include_str!("input")).iter().product();

    println!("Day 19");
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}
