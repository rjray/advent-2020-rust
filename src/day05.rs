// Advent 2020, Day 5

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
        .max()
        .unwrap();

    println!("{}", answer);
}

pub fn part2(input: String) {
    let mut passes: Vec<usize> = input
        .lines()
        .map(|s| {
            s.replace("B", "1")
                .replace("F", "0")
                .replace("R", "1")
                .replace("L", "0")
        })
        .map(|s| usize::from_str_radix(&s, 2).unwrap())
        .collect();
    passes.sort_unstable();

    // The "windows" approach taken from
    // https://github.com/timvisee/advent-of-code-2020/blob/master/day05b/src/main.rs
    let answer = passes.windows(2).find(|p| p[0] == p[1] - 2).unwrap()[0] + 1;

    println!("{}", answer);
}
