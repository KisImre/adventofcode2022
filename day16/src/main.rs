// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::{BTreeSet, HashMap};

use itertools::Itertools;
use regex::Regex;

struct Valve {
    name: String,
    flow_rate: u64,
    tunnels: Vec<String>,
    paths: Option<HashMap<String, u64>>,
}

impl Valve {
    fn new(name: String, flow_rate: u64, tunnels: Vec<String>) -> Self {
        Self {
            name,
            flow_rate,
            tunnels,
            paths: None,
        }
    }

    fn find_max(
        &self,
        valves: &HashMap<String, Valve>,
        mut minutes_left: u64,
        opened_valves: &mut Vec<String>,
    ) -> u64 {
        if minutes_left == 0 {
            return 0;
        }

        minutes_left -= 1;

        let self_pressure = self.flow_rate * minutes_left;

        let mut next_max_max = 0;

        if minutes_left != 0 && valves.len() != opened_valves.len() {
            opened_valves.push(self.name.clone());

            if let Some(paths) = &self.paths {
                for (name, dist) in paths {
                    if !opened_valves.contains(name) && minutes_left > *dist {
                        let next_max =
                            valves[name].find_max(valves, minutes_left - dist, opened_valves);

                        if next_max_max < next_max {
                            next_max_max = next_max;
                        }
                    }
                }
            }
            opened_valves.pop();
        }

        self_pressure + next_max_max
    }
}

fn parse_input(input: &str) -> HashMap<String, Valve> {
    let pattern = Regex::new(
        "Valve ([A-Z]{2}) has flow rate=([0-9]+); tunnel[s]? lead[s]? to valve[s]? ([A-Z, ]+)$",
    )
    .unwrap();

    let mut valves = HashMap::new();

    for line in input.split("\n") {
        let elements = pattern.captures(line).unwrap();

        let name = elements[1].to_string();
        let flow_rate = elements[2].parse::<u64>().unwrap();
        let tunnels = elements[3].split(", ").map(|s| s.to_string()).collect();

        let valve = Valve::new(name.clone(), flow_rate, tunnels);

        valves.insert(name, valve);
    }

    let flow_rates: HashMap<String, u64> = valves
        .iter()
        .map(|(name, valve)| (name.clone(), valve.flow_rate))
        .collect();

    let mut all_distances = HashMap::new();
    // Calculate distance to significant valves
    for (name, _valve) in valves
        .iter()
        .filter(|(name, valve)| *name == "AA" || valve.flow_rate != 0)
    {
        all_distances.insert(name.clone(), dijkstra(&valves, name.clone()));
    }

    for (name, valve) in &mut valves {
        if let Some(paths) = all_distances.get(name) {
            valve.paths = Some(
                paths
                    .iter()
                    .filter(|(n, _dist)| *n != name && flow_rates[*n] != 0)
                    .map(|(n, v)| (n.clone(), *v))
                    .collect(),
            );
        }
    }

    valves.retain(|n, v| n == "AA" || v.flow_rate != 0);

    valves
}

fn dijkstra(valves: &HashMap<String, Valve>, source: String) -> HashMap<String, u64> {
    // https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
    let mut distances: HashMap<String, u64> = valves
        .iter()
        .map(|(name, _valve)| (name.clone(), u64::MAX - 1000))
        .collect();

    let mut q: BTreeSet<String> = valves.iter().map(|(name, _valve)| name.clone()).collect();

    distances.insert(source.clone(), 0);

    loop {
        let min = q.iter().min_by(|a, b| distances[*a].cmp(&distances[*b]));

        if let Some(min_pos) = min {
            let u = min_pos.clone();
            q.remove(&u);

            for v in valves[&u].tunnels.iter().filter(|v| q.contains(*v)) {
                let alt = distances[&u] + 1;
                if alt < distances[v] {
                    distances.insert(v.clone(), alt);
                }
            }
        } else {
            break;
        }
    }

    distances
}

fn part1(input: &str) -> u64 {
    let valves = parse_input(input);

    let mut opened_valves = Vec::new();
    opened_valves.push(String::from("AA"));
    valves["AA"].find_max(&valves, 31, &mut opened_valves)
}

fn part2(input: &str) -> u64 {
    let valves = parse_input(input);

    let valve_names = valves.keys().map(|n| n.clone()).collect::<Vec<_>>();
    let count = valve_names.len();

    let mut max_res = 0;

    for split in 0..=(count / 2) {
        for selected_valve_names in valve_names.iter().combinations(split).collect::<Vec<_>>() {
            let mut opened_valves_a = selected_valve_names
                .iter()
                .map(|n| (*n).clone())
                .collect::<Vec<_>>();

            let res_a = valves["AA"].find_max(&valves, 27, &mut opened_valves_a);

            let mut opened_valves_b = valve_names
                .iter()
                .filter(|n| !selected_valve_names.contains(n))
                .map(|n| (*n).clone())
                .collect::<Vec<_>>();

            let res_b = valves["AA"].find_max(&valves, 27, &mut opened_valves_b);

            max_res = max_res.max(res_a + res_b);
        }
    }

    max_res
}

fn main() {
    assert_eq!(1651, part1(include_str!("input_test")));
    assert_eq!(1707, part2(include_str!("input_test")));

    let part1 = part1(include_str!("input"));
    let part2 = part2(include_str!("input"));

    println!("Day 16");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
