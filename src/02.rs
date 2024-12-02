use std::env;

fn main() {
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    let inputfile = &args[1];
    let input = std::fs::read_to_string(inputfile).unwrap();
    part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let i = parse_input(input);
    let mut safe = 0;
    for line in i {
        if is_safe(&line) {
            safe += 1;
        }
    }
    println!("Part 1: {}", safe);
}

fn part2(input: &str) {
    let i = parse_input(input);
    let mut safe = 0;
    for line in i {
        for skip in 0..line.len() {
            let newline: Vec<i64> = line
                .iter()
                .enumerate()
                .filter(|&(x, _)| x != skip)
                .map(|(_, &x)| x)
                .collect();
            if is_safe(&newline) {
                safe += 1;
                break;
            }
        }
    }
    println!("Part 2: {}", safe);
}

fn is_safe(line: &[i64]) -> bool {
    let mut all_inc = true;
    let mut all_dec = true;
    let mut all_safe = true;
    for j in 0..(line.len() - 1) {
        let d = line[j + 1] - line[j];
        let a = d.abs();
        if d > 0 {
            all_dec = false;
        } else if d < 0 {
            all_inc = false;
        }
        if a < 1 || a > 3 {
            all_safe = false;
        }
    }
    (all_inc || all_dec) && all_safe
}

fn parse_int(s: &str) -> i64 {
    s.parse().unwrap()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let mut a: Vec<Vec<i64>> = Vec::new();
    for line in input.lines() {
        let l = line.split_whitespace().map(parse_int).collect();
        a.push(l);
    }
    a
}
