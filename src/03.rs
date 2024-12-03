use regex::Regex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputfile = &args[1];
    let input = std::fs::read_to_string(inputfile).unwrap();
    part1(&input);
    part2(&input);
}

enum Cmd {
    Mul(i64, i64),
    Do,
    Dont,
}

fn parse_cmds(input: &str) -> Vec<Cmd> {
    let re = Regex::new(r"(mul\((\d+)\,(\d+)\)|do\(\)|don't\(\))").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let cmd = caps.get(1).map_or("do()", |m| m.as_str());
            let a: i64 = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());
            let b: i64 = caps.get(3).map_or(0, |m| m.as_str().parse().unwrap());

            if cmd.starts_with("don't") {
                Cmd::Dont
            } else if cmd.starts_with("do(") {
                Cmd::Do
            } else {
                Cmd::Mul(a, b)
            }
        })
        .collect()
}

fn run_cmds(cmds: Vec<Cmd>, always_enabled: bool) -> i64 {
    let mut sum = 0;
    let mut enabled = true;

    for cmd in cmds {
        match cmd {
            Cmd::Do => enabled = true,
            Cmd::Dont => enabled = false,
            Cmd::Mul(a, b) => {
                sum = if always_enabled || enabled {
                    sum + a * b
                } else {
                    sum
                }
            }
        }
    }
    sum
}

fn part1(input: &str) {
    let sum = run_cmds(parse_cmds(&input), true);
    println!("Part 1: {}", sum)
}

fn part2(input: &str) {
    let sum = run_cmds(parse_cmds(&input), false);
    println!("Part 1: {}", sum)
}
