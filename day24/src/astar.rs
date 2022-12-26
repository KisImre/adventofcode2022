// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::{
    collections::{BinaryHeap, HashMap},
    hash::Hash,
    ops::Deref,
};

fn reconstruct_path<P: Eq + Hash + Copy>(came_from: HashMap<P, P>, mut current: P) -> Vec<P> {
    let mut total_path = vec![current];
    while let Some(next) = came_from.keys().find(|p| **p == current) {
        current = came_from[next];
        total_path.push(current);
    }
    total_path.reverse();

    total_path
}

struct MinWrapper<P> {
    value: P,
    weight: usize,
}

impl<P> MinWrapper<P> {
    fn new(value: P, weight: usize) -> Self {
        MinWrapper { value, weight }
    }
}

impl<P> PartialEq for MinWrapper<P> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<P> Eq for MinWrapper<P> {}

impl<P> PartialOrd for MinWrapper<P> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl<P> Ord for MinWrapper<P> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<P> Deref for MinWrapper<P> {
    type Target = P;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

pub fn a_star<
    P: Eq + Hash + Copy,
    H: Fn(&P) -> usize,
    D: Fn(&P, &P) -> usize,
    N: Fn(&P) -> Vec<P>,
    OK: Fn(&P, &P) -> bool,
>(
    start: P,
    goal: P,
    heuristic_func: H,
    distance_func: D,
    neighbors: N,
    is_goal: OK,
) -> Vec<P> {
    let mut open_set = BinaryHeap::new();
    open_set.push(MinWrapper::new(start.clone(), heuristic_func(&start)));

    let mut came_from = HashMap::new();

    let mut best_distance_from_start = HashMap::new();
    best_distance_from_start.insert(start, 0);

    while let Some(current) = open_set.pop() {
        if is_goal(&current, &goal) {
            return reconstruct_path(came_from, *current);
        }

        for neighbor in neighbors(&current) {
            let tentative_score =
                best_distance_from_start[&current] + distance_func(&current, &neighbor);

            let best_neighbor_dist_from_start = *best_distance_from_start
                .get(&neighbor)
                .unwrap_or(&usize::MAX);

            if tentative_score < best_neighbor_dist_from_start {
                came_from.insert(neighbor.clone(), *current);
                best_distance_from_start.insert(neighbor, tentative_score);
                open_set.push(MinWrapper {
                    value: neighbor,
                    weight: tentative_score + heuristic_func(&neighbor),
                });
            }
        }
    }

    return Vec::new();
}
