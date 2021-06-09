// Advent 2020, Day 6

use std::collections::HashSet;
use std::iter::FromIterator;

// Starting with the Clojure approach, which uses sets.

pub fn part1(input: String) {
    let answer: usize = input
        .split("\n\n")
        .map(|group| group.replace("\n", "").as_bytes().to_vec())
        .map(|v| HashSet::from_iter(v))
        .map(|h: HashSet<u8>| h.len())
        .sum::<usize>();

    println!("{}", answer);
}

pub fn part2(input: String) {
    let answer: usize = input
        .split("\n\n")
        .map(|group| group_count(&group))
        .sum::<usize>();

    println!("{}", answer);
}

fn group_count(group: &str) -> usize {
    group
        .lines()
        .map(|l| l.bytes())
        .map(|v| HashSet::from_iter(v))
        .reduce(|a: HashSet<u8>, b: HashSet<u8>| a.intersection(&b).cloned().collect())
        .unwrap()
        .len()
}
