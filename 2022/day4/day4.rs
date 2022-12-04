/*
Advent of Code 2022: Day 4 Camp Cleanup

https://adventofcode.com/2022/day/4
*/

use std::cmp;
use std::convert::TryInto;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Copy, Clone, Debug)]
struct Range(i32, i32);

#[derive(Debug)]
struct PairRange(Range, Range);

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.0, self.1)
    }
}

fn readlines(filepath: &Path) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let binding = fs::read_to_string(filepath).unwrap();
    for line in binding.lines() {
        lines.push(String::from(line));
    }

    lines
}

fn parse_lines(lines: &Vec<String>) -> Vec<PairRange> {
    let mut ranges: Vec<Range> = Vec::<Range>::with_capacity(2);
    let mut pairs: Vec<PairRange> = Vec::<PairRange>::new();

    for line in lines {
        let sections = line.split(",");
        for section in sections {
            let assignments: Vec<&str> = section.split("-").collect();
            let range = Range(
                assignments[0].parse().unwrap(),
                assignments[1].parse().unwrap(),
            );
            ranges.push(range);
            if ranges.len() == 2 {
                pairs.push(PairRange(ranges[0], ranges[1]));
                ranges = vec![];
            }
        }
    }

    pairs
}

fn _draw_section(section: &Range) {
    let max_value: usize = cmp::max(section.0, section.1).try_into().unwrap();
    let mut vec: Vec<String> = vec!["."; max_value + 1]
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let min_value: usize = cmp::min(section.0, section.1).try_into().unwrap();
    // max_value inclusive here with =
    for i in min_value..=max_value {
        let binding: Vec<char> = i.to_string().chars().collect();
        let s = binding.into_iter().collect();
        vec[i] = s;
    }

    let s: String = vec.join("");
    println!("{:} {}", &s[1..], section);
}

fn range_to_vec(range: Range) -> Vec<i32> {
    let mut vec: Vec<i32> = Vec::new();
    let start: i32 = range.0;
    let end: i32 = range.1;

    // end inclusive here with =
    for i in start..=end {
        vec.push(i);
    }

    vec
}

fn is_fully_contained(mut arr1: Vec<i32>, mut arr2: Vec<i32>) -> bool {
    let mut nb_same: usize = 0;
    let temp: Vec<i32>;

    if arr1.len() < arr2.len() {
        temp = arr1;
        arr1 = arr2;
        arr2 = temp;
    }

    if arr2.len() > arr1.len() {
        return false;
    }

    if arr2.len() == 0 && arr1.len() == 0 {
        return false;
    }

    let mut j: usize = 0;
    let size: usize = arr1.len();
    for i in 0..size {
        if j < arr2.len() {
            if arr1[i] == arr2[j] {
                nb_same += 1;
                j += 1;
            }
        }
    }

    if nb_same == arr2.len() {
        return true;
    }

    false
}

fn camp_part1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let pairs: Vec<PairRange> = parse_lines(&lines);

    for pair in &pairs {
        /*_draw_section(&pair.0);
        _draw_section(&pair.1);
        println!();*/
        let contained = is_fully_contained(range_to_vec(pair.0), range_to_vec(pair.1));
        if contained {
            result += 1;
        }
    }

    result
}

fn is_overlapped(arr1: Vec<i32>, arr2: Vec<i32>) -> bool {
    let size: usize = arr1.len();
    for i in 0..size {
        if arr2.contains(&arr1[i]) {
            return true;
        }
    }

    false
}

fn camp_part2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let pairs: Vec<PairRange> = parse_lines(&lines);

    for pair in &pairs {
        let overlapped = is_overlapped(range_to_vec(pair.0), range_to_vec(pair.1));
        if overlapped {
            result += 1;
        }
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
    result = camp_part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = camp_part2(&lines);
    println!("Result Part 2: {:?}", result);
}
