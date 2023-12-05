/*
Advent of Code 2023: Day 1 Trebuchet?!

https://adventofcode.com/2023/day/1
*/

use std::convert::TryInto;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn readlines(filepath: &Path) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let binding = fs::read_to_string(filepath).unwrap();
    for line in binding.lines() {
        lines.push(String::from(line));
    }

    lines
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut result: u32 = 0;

    for line in lines.iter() {
        let pair: Vec<_> = line
            .chars()
            .into_iter()
            .filter_map(|c| {
                if c.to_digit(10).is_some() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        result += format!("{}{}", pair[0], pair[pair.len() - 1])
            .parse::<u32>()
            .unwrap();
    }

    result.try_into().unwrap()
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut result: u32 = 0;

    for line in lines.iter() {
        let pair: Vec<_> = line
            .replace("one", "o1e") // -> "4onesix" -> "4o1esix" -> "46"
            .replace("two", "t2o")
            .replace("three", "t3e")
            .replace("four", "f4r")
            .replace("five", "f5e")
            .replace("six", "s6x")
            .replace("seven", "s7n")
            .replace("eight", "e8t")
            .replace("nine", "n9e")
            .chars()
            .into_iter()
            .filter_map(|c| {
                if c.to_digit(10).is_some() {
                    Some(c)
                } else {
                    None
                }
            })
            .collect();
        result += format!("{}{}", pair[0], pair[pair.len() - 1])
            .parse::<u32>()
            .unwrap();
    }

    result.try_into().unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("usage: {} input.txt", &args[0]);
        process::exit(1);
    }
    let filepath = &args[1];
    let mut result: i32;
    let lines = readlines(Path::new(filepath));
    result = part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = part2(&lines);
    println!("Result Part 2: {:?}", result);
}
