// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn FnMut(u64) -> u64>,
    divisor: u64,
    targets: (usize, usize),
    throws: usize,
}

impl Monkey {
    fn new<O: FnMut(u64) -> u64 + 'static>(
        items: Vec<u64>,
        operation: O,
        divisor: u64,
        targets: (usize, usize),
    ) -> Self {
        Self {
            items,
            operation: Box::new(operation),
            throws: 0,
            divisor,
            targets,
        }
    }

    fn throws(&mut self, simplifier: u64, divisor: u64) -> Vec<(u64, usize)> {
        self.items
            .iter()
            .map(|w| ((self.operation)(*w) / divisor) % simplifier)
            .map(|w| {
                (
                    w,
                    if w % self.divisor == 0 {
                        self.targets.0
                    } else {
                        self.targets.1
                    },
                )
            })
            .collect()
    }

    fn add(&mut self, worry: u64) {
        self.items.push(worry);
    }

    fn clear(&mut self) {
        self.throws += self.items.len();
        self.items.clear();
    }
}

fn part12(mut monkeys: Vec<Monkey>, turns: u32, divisor: u64) -> usize {
    let simplifier = monkeys.iter().map(|m| m.divisor).product();
    for _turn in 0..turns {
        for monkey_index in 0..monkeys.len() {
            let throws = monkeys[monkey_index].throws(simplifier, divisor);

            for (worry, to_monkey) in throws {
                monkeys[to_monkey as usize].add(worry);
            }

            monkeys[monkey_index].clear();
        }
    }

    let mut throws = monkeys.iter().map(|t| t.throws).collect::<Vec<usize>>();
    throws.sort();
    throws.reverse();

    throws[0] * throws[1]
}

fn main() {
    let mut monkeys = Vec::new();
    monkeys.push(Monkey::new(vec![79, 98], |old| old * 19, 23, (2, 3)));
    monkeys.push(Monkey::new(vec![54, 65, 75, 74], |old| old + 6, 19, (2, 0)));
    monkeys.push(Monkey::new(vec![79, 60, 97], |old| old * old, 13, (1, 3)));
    monkeys.push(Monkey::new(vec![74], |old| old + 3, 17, (0, 1)));
    assert_eq!(10605, part12(monkeys, 20, 3));

    let mut monkeys = Vec::new();
    monkeys.push(Monkey::new(vec![79, 98], |old| old * 19, 23, (2, 3)));
    monkeys.push(Monkey::new(vec![54, 65, 75, 74], |old| old + 6, 19, (2, 0)));
    monkeys.push(Monkey::new(vec![79, 60, 97], |old| old * old, 13, (1, 3)));
    monkeys.push(Monkey::new(vec![74], |old| old + 3, 17, (0, 1)));
    assert_eq!(2713310158, part12(monkeys, 10000, 1));

    let mut monkeys = Vec::new();
    monkeys.push(Monkey::new(vec![72, 97], |old| old * 13, 19, (5, 6)));
    monkeys.push(Monkey::new(
        vec![55, 70, 90, 74, 95],
        |old| old * old,
        7,
        (5, 0),
    ));
    monkeys.push(Monkey::new(vec![74, 97, 66, 57], |old| old + 6, 17, (1, 0)));
    monkeys.push(Monkey::new(vec![86, 54, 53], |old| old + 2, 13, (1, 2)));
    monkeys.push(Monkey::new(
        vec![50, 65, 78, 50, 62, 99],
        |old| old + 3,
        11,
        (3, 7),
    ));
    monkeys.push(Monkey::new(vec![90], |old| old + 4, 2, (4, 6)));
    monkeys.push(Monkey::new(
        vec![88, 92, 63, 94, 96, 82, 53, 53],
        |old| old + 8,
        5,
        (4, 7),
    ));
    monkeys.push(Monkey::new(
        vec![70, 60, 71, 69, 77, 70, 98],
        |old| old * 7,
        3,
        (2, 3),
    ));
    let part1 = part12(monkeys, 20, 3);

    let mut monkeys = Vec::new();
    monkeys.push(Monkey::new(vec![72, 97], |old| old * 13, 19, (5, 6)));
    monkeys.push(Monkey::new(
        vec![55, 70, 90, 74, 95],
        |old| old * old,
        7,
        (5, 0),
    ));
    monkeys.push(Monkey::new(vec![74, 97, 66, 57], |old| old + 6, 17, (1, 0)));
    monkeys.push(Monkey::new(vec![86, 54, 53], |old| old + 2, 13, (1, 2)));
    monkeys.push(Monkey::new(
        vec![50, 65, 78, 50, 62, 99],
        |old| old + 3,
        11,
        (3, 7),
    ));
    monkeys.push(Monkey::new(vec![90], |old| old + 4, 2, (4, 6)));
    monkeys.push(Monkey::new(
        vec![88, 92, 63, 94, 96, 82, 53, 53],
        |old| old + 8,
        5,
        (4, 7),
    ));
    monkeys.push(Monkey::new(
        vec![70, 60, 71, 69, 77, 70, 98],
        |old| old * 7,
        3,
        (2, 3),
    ));
    let part2 = part12(monkeys, 10000, 1);

    println!("Day 11");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
