/*
Advent of Code 2022: Day 6 Tuning Trouble

https://adventofcode.com/2022/day/6
*/

use std::collections::HashSet;
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

fn parse_stream(lines: &Vec<String>) -> Vec<char> {
    let mut stream: Vec<char> = vec![];
    for line in lines {
        stream.append(&mut line.chars().collect::<Vec<_>>());
    }

    stream
}

fn contains_repeated_char(chars: &[char]) -> bool {
    let mut char_set = HashSet::new();
    for c in chars {
        if char_set.contains(c) {
            return true;
        }
        char_set.insert(c);
    }

    return false;
}

fn find_index_marker(stream: Vec<char>, step_window: usize) -> usize {
    let mut index: usize = 0;

    for i in 0..=stream.len() - step_window {
        let chunk = &stream[i..(i + step_window)];
        if !contains_repeated_char(chunk) {
            index = i;
            break;
        }
    }

    index
}

fn tuning_part1(lines: &Vec<String>) -> i32 {
    let step_window: usize = 4; // 4 distinct chars for marker
    let stream: Vec<char> = parse_stream(&lines);
    let index: usize = find_index_marker(stream, step_window);

    (index + step_window).try_into().unwrap()
}

fn tuning_part2(lines: &Vec<String>) -> i32 {
    let step_window: usize = 14; // 14 distinct chars for marker
    let stream: Vec<char> = parse_stream(&lines);
    let index: usize = find_index_marker(stream, step_window);

    (index + step_window).try_into().unwrap()
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
    result = tuning_part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = tuning_part2(&lines);
    println!("Result Part 2: {:?}", result);
}
