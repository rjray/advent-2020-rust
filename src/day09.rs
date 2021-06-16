// Advent 2020, Day 9

use itertools::Itertools;

fn find_outlier(numbers: &Vec<usize>) -> usize {
    let winsize = if numbers.len() <= 20 { 5 } else { 25 };

    numbers
        .windows(winsize + 1)
        .find(|win| {
            let mut copy = win.to_vec();
            let target = copy.pop().unwrap();
            let mut it = copy.iter().combinations(2);

            !it.any(|i| i[0] + i[1] == target)
        })
        .unwrap()[winsize]
}

pub fn part1(input: String) {
    let numbers: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();

    println!("{}", find_outlier(&numbers));
}

pub fn part2(input: String) {
    let numbers: Vec<usize> = input.lines().map(|i| i.parse().unwrap()).collect();

    let target = find_outlier(&numbers);
    let (mut tail, mut head, mut tot) = (0, 1, numbers[0] + numbers[1]);
    while tot != target {
        while tot < target {
            head += 1;
            tot += numbers[head];
        }
        while tot > target {
            tot -= numbers[tail];
            tail += 1;
        }
    }

    let set = &numbers[tail..=head];
    let answer = set.iter().min().unwrap() + set.iter().max().unwrap();
    println!("{}", answer);
}
