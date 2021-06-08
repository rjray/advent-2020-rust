// Advent 2020, Day 5

use itertools::Itertools;

pub fn part1(input: String) {
    let answer: usize = input
        .lines()
        .map(|s| {
            s.replace("B", "1")
                .replace("F", "0")
                .replace("R", "1")
                .replace("L", "0")
        })
        .map(|s| usize::from_str_radix(&s, 2).unwrap())
        .sorted()
        .last()
        .unwrap();

    println!("{}", answer);
}

pub fn part2(input: String) {
    let passes: Vec<usize> = input
        .lines()
        .map(|s| {
            s.replace("B", "1")
                .replace("F", "0")
                .replace("R", "1")
                .replace("L", "0")
        })
        .map(|s| usize::from_str_radix(&s, 2).unwrap())
        .sorted()
        .collect();

    let mut counter = 0;
    let answer = loop {
        if counter == (passes.len() - 1) {
            panic!("no seat found");
        } else if passes[counter] == (passes[counter + 1] - 2) {
            break passes[counter] + 1;
        }

        counter += 1;
    };

    println!("{}", answer);
}
