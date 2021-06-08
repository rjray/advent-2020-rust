// Advent 2020, Day 4

use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

const REQUIRED: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub fn part1(input: String) {
    let answer: usize = input
        .split("\n\n")
        .map(|block| {
            block
                .split_ascii_whitespace()
                .map(|part| part.split_once(":").unwrap())
                .collect::<HashMap<_, _>>()
        })
        .filter(|pp| REQUIRED.iter().all(|f| pp.contains_key(f)))
        .count();

    println!("{}", answer);
}

pub fn part2(input: String) {
    let answer: usize = input
        .split("\n\n")
        .map(|block| {
            block
                .split_ascii_whitespace()
                .map(|part| part.split_once(":").unwrap())
                .collect::<HashMap<_, _>>()
        })
        .filter(|pp| REQUIRED.iter().all(|f| pp.contains_key(f)))
        .filter(|pp| pp.iter().all(|(k, v)| is_valid(k, v)))
        .count();

    println!("{}", answer);
}

fn is_valid(key: &str, val: &str) -> bool {
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
        static ref YEAR_RE: Regex = Regex::new(r"^\d{4}$").unwrap();
        static ref HCL_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        static ref HGT_RE: Regex = Regex::new(r"^(\d+)(in|cm)$").unwrap();
    }
    match key {
        "byr" => YEAR_RE.is_match(&val) && between(&val, 1920, 2002),
        "iyr" => YEAR_RE.is_match(&val) && between(&val, 2010, 2020),
        "eyr" => YEAR_RE.is_match(&val) && between(&val, 2020, 2030),
        "hgt" => {
            let valid: bool;

            if !HGT_RE.is_match(&val) {
                valid = false;
            } else {
                let pair = HGT_RE.captures(&val).unwrap();
                match &pair[2] {
                    "cm" => valid = between(&pair[1], 150, 193),
                    "in" => valid = between(&pair[1], 59, 76),
                    _ => valid = false,
                }
            }

            valid
        }
        "hcl" => HCL_RE.is_match(&val),
        "ecl" => COLORS.iter().any(|v| v == &val),
        "pid" => PID_RE.is_match(&val),
        "cid" => true,
        _ => panic!("unknown field"),
    }
}

fn between(year: &str, lo: usize, hi: usize) -> bool {
    let yr = year.parse::<usize>().unwrap();
    yr >= lo && yr <= hi
}
