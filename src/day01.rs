// Advent 2020, Day 1

// Based on my Clojure solution, with details filled in from
// https://github.com/timvisee/advent-of-code-2020/blob/master/day01a/src/main.rs

use itertools::Itertools;

pub fn part1(input: String) {
    let answer: usize = input
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .combinations(2)
        .filter(|i| i.iter().sum::<usize>() == 2020)
        .next()
        .map(|i| i.iter().product::<usize>())
        .unwrap();

    println!("{}", answer);
}

pub fn part2(input: String) {
    let answer: usize = input
        .lines()
        .map(|i| i.parse::<usize>().unwrap())
        .combinations(3)
        .filter(|i| i.iter().sum::<usize>() == 2020)
        .next()
        .map(|i| i.iter().product::<usize>())
        .unwrap();

    println!("{}", answer);
}
