// Advent 2020, Day 10

pub fn part1(input: String) {
    let mut numbers: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();
    numbers.sort();

    let mut counts: [usize; 4] = [0; 4];

    numbers
        .windows(2)
        .for_each(|pair| counts[pair[1] - pair[0]] += 1);

    counts[numbers[0]] += 1;
    counts[3] += 1;

    println!("{}", counts[1] * counts[3]);
}

pub fn part2(input: String) {
    let mut numbers: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();
    numbers.sort();

    let answer: usize = numbers
        .windows(2)
        .collect::<Vec<_>>()
        .split(|pair| pair[1] - pair[0] == 3)
        .map(|grp| match grp.len() {
            4 => 7,
            3 => 4,
            2 => 2,
            _ => 1,
        })
        .product::<usize>()
        * 2;

    println!("{}", answer);
}
