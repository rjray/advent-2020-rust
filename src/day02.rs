// Advent 2020, Day 2

// Started with my Clojure solution. The behavior of group_by() in Rust was not
// what I needed, so I turned to this for guidance:
// https://github.com/timvisee/advent-of-code-2020/blob/master/day02a/src/main.rs

use regex::Regex;

pub fn part1(input: String) {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(\w+)$").unwrap();

    let answer: usize = input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .filter(|cap| {
            let num = cap[4].matches(&cap[3]).count();
            num >= cap[1].parse().unwrap() && num <= cap[2].parse().unwrap()
        })
        .count();

    println!("{}", answer);
}

// For part 2, this is much closer to the Clojure version.
pub fn part2(input: String) {
    let re = Regex::new(r"^(\d+)-(\d+)\s(\w):\s(\w+)$").unwrap();

    let answer: usize = input
        .lines()
        .map(|line| re.captures(line).unwrap())
        .filter(|cap| {
            // The "chars" method is safe to use here since all input is ASCII.
            let chs: Vec<char> = cap[4].chars().collect();
            let chlo: char = chs[cap[1].parse::<usize>().unwrap() - 1];
            let chhi: char = chs[cap[2].parse::<usize>().unwrap() - 1];
            let ch3: Vec<char> = cap[3].chars().collect();
            let match_lo: bool = chlo == ch3[0];
            let match_hi: bool = chhi == ch3[0];

            (match_lo || match_hi) && (!(match_lo && match_hi))
        })
        .count();

    println!("{}", answer);
}
