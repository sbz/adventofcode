/*
Advent of Code 2022: Day 1 Calorie Counting

https://adventofcode.com/2022/day/1
*/

use std::env;
use std::fs;
use std::path::Path;
use std::process;

fn readlines(filepath: &Path) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let binding = fs::read_to_string(filepath).unwrap();
    for line in binding.split("\n") {
        lines.push(String::from(line));
    }

    lines
}

fn calorie_part1(lines: &Vec<String>) -> i32 {
    let mut calories: Vec<i32> = Vec::new();
    let mut acc: Vec<i32> = Vec::new();

    for line in lines.iter() {
        if line.is_empty() {
            calories.push(acc.iter().sum());
            acc = Vec::new();
        } else {
            acc.push(line.parse().unwrap());
        }
    }
    calories.push(acc.iter().sum());

    *calories.iter().max().unwrap()
}

fn calorie_part2(lines: &Vec<String>) -> i32 {
    let mut calories: Vec<i32> = Vec::new();
    let mut acc: Vec<i32> = Vec::new();
    let mut result: i32 = 0;

    for line in lines {
        if line.is_empty() {
            calories.push(acc.iter().sum());
            acc = Vec::new();
        } else {
            acc.push(line.parse().unwrap());
        }
    }
    calories.push(acc.iter().sum());

    calories.sort_by(|a, b| b.cmp(&a));

    for c in &calories[0..3] {
        result += c;
    }

    result
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
    result = calorie_part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = calorie_part2(&lines);
    println!("Result Part 2: {:?}", result);
}
