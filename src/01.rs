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
    let (mut a, mut b) = parse_input(input);
    a.sort();
    b.sort();

    let mut dist = 0;
    for i in 0..a.len() {
        dist += (a[i] - b[i]).abs();
    }

    println!("Part 1: {}", dist);
}

fn part2(input: &str) {
    let (a, b) = parse_input(input);

    let mut dist = 0;
    for i in a {
        let n = b.iter().filter(|&x| *x == i).count() as i64;
        dist += i * n;
    }

    println!("Part 2: {}", dist);
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<i64>) {
    let x: Vec<Vec<i64>> = input.lines().map(parse_line).collect();

    let mut a: Vec<i64> = Vec::new();
    let mut b: Vec<i64> = Vec::new();

    for line in &x {
        a.push(line[0]);
        b.push(line[1]);
    }

    (a, b)
}
