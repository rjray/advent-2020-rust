// Advent 2020, Day 2

// Started with my Clojure solution. Turned to this for guidance on parsing the
// input into a usable data structure:
// https://github.com/timvisee/advent-of-code-2020/blob/master/day03a/src/main.rs

fn count_collisions(map: &Vec<&[u8]>, slope: (usize, usize)) -> usize {
    let (xs, ys) = slope;
    let height = map.len();
    let width = map[0].len();

    let mut x = 0;
    let mut y = 0;
    let mut trees = 0;

    let answer = loop {
        y += ys;
        x = (x + xs) % width;

        if y >= height {
            break trees;
        }

        if map[y][x] == b'#' {
            trees += 1;
        }
    };

    answer
}

pub fn part1(input: String) {
    let map: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let answer = count_collisions(&map, (3, 1));

    println!("{}", answer);
}

pub fn part2(input: String) {
    let map: Vec<&[u8]> = input.lines().map(|l| l.as_bytes()).collect();
    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let answer = slopes
        .iter()
        .map(|s| count_collisions(&map, *s))
        .product::<usize>();

    println!("{}", answer);
}
