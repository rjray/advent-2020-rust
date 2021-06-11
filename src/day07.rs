// Advent 2020, Day 7

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn contents(items: &str) -> HashMap<String, usize> {
    lazy_static! {
        static ref MATCH: Regex = Regex::new(r"(\d+) (.*?) bags?[.,]").unwrap();
    }
    let mut stuff: HashMap<String, usize> = HashMap::new();

    if items != "no other bags." {
        MATCH.captures_iter(items).for_each(|cap| {
            let count = cap[1].parse::<usize>().unwrap();
            let color = String::from(&cap[2]);

            stuff.insert(color, count);
        });
    }

    return stuff;
}

fn parse_rules(input: String) -> HashMap<String, HashMap<String, usize>> {
    let mut rules: HashMap<String, HashMap<String, usize>> = HashMap::new();

    input
        .lines()
        .map(|l| l.split_once(" bags contain ").unwrap())
        .for_each(|(bag, items)| {
            rules.insert(String::from(bag), contents(items));
        });

    return rules;
}

fn can_contain(
    rules: &HashMap<String, HashMap<String, usize>>,
    choice: &str,
    target: &str,
) -> bool {
    let contains: Vec<&String> = rules.get(choice).unwrap().keys().collect();

    if contains.is_empty() {
        return false;
    } else if contains.iter().any(|&x| x == target) {
        return true;
    } else {
        return contains.iter().any(|&x| can_contain(&rules, x, target));
    }
}

fn find_colors(rules: &HashMap<String, HashMap<String, usize>>, target: &str) -> Vec<String> {
    rules
        .keys()
        .filter(|choice| can_contain(rules, choice, target))
        .map(|s| String::from(s))
        .collect::<Vec<_>>()
}

pub fn part1(input: String) {
    let rules = parse_rules(input);

    println!("{}", find_colors(&rules, "shiny gold").len());
}

fn inside(rules: &HashMap<String, HashMap<String, usize>>, target: &str) -> usize {
    let content = rules.get(target).unwrap();
    let contains: Vec<&String> = content.keys().collect();

    if contains.is_empty() {
        1
    } else {
        contains
            .iter()
            .map(|k| inside(rules, k) * content.get(*k).unwrap())
            .sum::<usize>()
            + 1
    }
}

pub fn part2(input: String) {
    let rules = parse_rules(input);

    println!("{}", inside(&rules, "shiny gold") - 1);
}
