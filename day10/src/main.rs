// Copyright (c) 2022, Kis Imre. All rights reserved.
// SPDX-License-Identifier: MIT

fn update_strength(strength: &mut i32, x: i32, cycle: u32, display: &mut Vec<String>) {
    if cycle > 240 {
        return;
    }

    if cycle >= 20 && (cycle - 20) % 40 == 0 {
        *strength += x * cycle as i32;
    }

    let pos = (cycle - 1) as i32;
    if ((pos % 40) - x).abs() <= 1 {
        display[(pos / 40) as usize].push('#');
    } else {
        display[(pos / 40) as usize].push('.');
    }
}

fn part12(input: &str) -> (i32, Vec<String>) {
    let mut display = vec![String::new(); 6];

    let mut strength: i32 = 0;
    let mut x: i32 = 1;
    let mut cycle: u32 = 1;

    update_strength(&mut strength, x, cycle, &mut display);
    for line in input.split("\n").filter(|l| !l.is_empty()) {
        let parts: Vec<&str> = line.split(" ").collect();
        if parts[0] == "noop" {
            cycle += 1;
            update_strength(&mut strength, x, cycle, &mut display);
        } else if parts[0] == "addx" {
            cycle += 1;

            update_strength(&mut strength, x, cycle, &mut display);

            x += parts[1].parse::<i32>().unwrap();
            cycle += 1;

            update_strength(&mut strength, x, cycle, &mut display);
        }
    }

    (strength, display)
}

fn main() {
    let (test1, test2) = part12(include_str!("input_test"));
    assert_eq!(13140, test1);
    assert_eq!(
        vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######....."
        ],
        test2
    );

    let (part1, part2) = part12(include_str!("input"));
    println!("Day 08");
    println!("Part 1: {}", part1);
    println!("Part 2:");
    for line in part2 {
        println!("{}", line);
    }
}
