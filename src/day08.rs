// Advent 2020, Day 8

// Too much of this was taken from:
// https://github.com/timvisee/advent-of-code-2020/blob/master/day08b/src/main.rs
//
// Not sure I learned what I needed to from this.

use std::mem;

pub fn part1(input: String) {
    let code: Vec<(&[u8], isize)> = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|bs| (&bs[0..3], atoi::atoi(&bs[4..]).unwrap()))
        .collect();

    let (mut seen, mut pc, mut acc) = (vec![], 0 as isize, 0);
    while !seen.contains(&pc) {
        seen.push(pc);
        match code[pc as usize] {
            (b"jmp", val) => pc += val - 1,
            (b"acc", val) => acc += val,
            _ => {}
        }
        pc += 1;
    }

    println!("{}", acc);
}

pub fn part2(input: String) {
    let mut code: Vec<(&[u8], i32)> = input
        .lines()
        .map(|line| line.as_bytes())
        .map(|bs| (&bs[0..3], atoi::atoi(&bs[4..]).unwrap()))
        .collect();

    let mut other_op: &[u8] = b"nop";
    for pc in run(&code).0 {
        if code[pc as usize].0 == b"jmp" {
            mem::swap(&mut code[pc as usize].0, &mut other_op);
            if let (_, Some(acc)) = run(&code) {
                println!("{}", acc);
                break;
            }
            mem::swap(&mut code[pc as usize].0, &mut other_op);
        }
    }
}

fn run(program: &[(&[u8], i32)]) -> (Vec<i32>, Option<i32>) {
    let (mut visited, mut pc, mut acc) = (Vec::with_capacity(64), 0 as i32, 0);
    while !visited.contains(&pc) {
        visited.push(pc);
        match program.get(pc as usize) {
            Some((b"acc", val)) => acc += val,
            Some((b"jmp", val)) => pc += *val - 1,
            None => return (visited, Some(acc)),
            _ => {}
        }
        pc += 1;
    }
    (visited, None)
}
