// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Operation {
    op1: String,
    op2: String,
    op_type: String,
}

impl Operation {
    fn new(value: &str) -> Self {
        let parts: Vec<&str> = value.split(" ").collect();

        Self {
            op1: String::from(parts[0]),
            op2: String::from(parts[2]),
            op_type: String::from(parts[1]),
        }
    }

    fn get_value(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        let op1 = monkeys.get(&self.op1).unwrap().get_value(monkeys);
        let op2 = monkeys.get(&self.op2).unwrap().get_value(monkeys);

        match self.op_type.as_str() {
            "+" => op1 + op2,
            "-" => op1 - op2,
            "*" => op1 * op2,
            "/" => op1 / op2,
            op => panic!("Invalid operation {}", op),
        }
    }

    fn is_unknown(&self, monkeys: &HashMap<String, Monkey>) -> bool {
        let op1 = monkeys.get(&self.op1).unwrap();
        let op2 = monkeys.get(&self.op2).unwrap();

        op1.is_unknown(monkeys) || op2.is_unknown(monkeys)
    }

    fn solve_unknown(&self, monkeys: &HashMap<String, Monkey>, res: i64) -> i64 {
        let op1 = monkeys.get(&self.op1).unwrap();
        let op2 = monkeys.get(&self.op2).unwrap();

        if op1.is_unknown(monkeys) {
            let next_res = match self.op_type.as_str() {
                "+" => res - op2.get_value(monkeys),
                "-" => res + op2.get_value(monkeys),
                "*" => res / op2.get_value(monkeys),
                "/" => res * op2.get_value(monkeys),
                "=" => op2.get_value(monkeys),
                op => panic!("Invalid operation {}", op),
            };

            op1.solve_unknowns(monkeys, next_res)
        } else {
            let next_res = match self.op_type.as_str() {
                "+" => res - op1.get_value(monkeys),
                "-" => op1.get_value(monkeys) - res,
                "*" => res / op1.get_value(monkeys),
                "/" => op1.get_value(monkeys) / res,
                "=" => op1.get_value(monkeys),
                op => panic!("Invalid operation {}", op),
            };
            op2.solve_unknowns(monkeys, next_res)
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    value: Option<i64>,
    operation: Option<Operation>,
    unknown: bool,
}

impl Monkey {
    fn new(value: &str) -> Self {
        let value_as_option_int = value.parse().ok();

        let operation = if !value_as_option_int.is_some() {
            Some(Operation::new(value))
        } else {
            None
        };

        Self {
            value: value_as_option_int,
            operation,
            unknown: false,
        }
    }

    fn get_value(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        if let Some(value) = self.value {
            value
        } else {
            self.operation.as_ref().unwrap().get_value(monkeys)
        }
    }

    fn is_unknown(&self, monkeys: &HashMap<String, Monkey>) -> bool {
        self.unknown
            || (self.value.is_none() && self.operation.as_ref().unwrap().is_unknown(monkeys))
    }

    fn solve_unknowns(&self, monkeys: &HashMap<String, Monkey>, res: i64) -> i64 {
        if self.unknown {
            res
        } else {
            self.operation.as_ref().unwrap().solve_unknown(monkeys, res)
        }
    }
}

fn part1(input: &str) -> i64 {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(": ").collect();
        monkeys.insert(String::from(parts[0]), Monkey::new(parts[1]));
    }

    monkeys["root"].get_value(&monkeys)
}

fn part2(input: &str) -> i64 {
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(": ").collect();
        monkeys.insert(String::from(parts[0]), Monkey::new(parts[1]));
    }

    let mut monkeys = monkeys.clone();

    monkeys
        .get_mut("root")
        .unwrap()
        .operation
        .as_mut()
        .unwrap()
        .op_type
        .replace_range(0..1, "=");
    monkeys.get_mut("humn").unwrap().unknown = true;
    monkeys.get_mut("humn").unwrap().value = None;

    monkeys["root"].solve_unknowns(&monkeys, 0)
}

fn main() {
    assert_eq!(152, part1(include_str!("input_test")));
    assert_eq!(301, part2(include_str!("input_test")));

    let part1 = part1(include_str!("input"));
    let part2 = part2(include_str!("input"));

    println!("Day 21");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2); // too high 9910093046258
}
