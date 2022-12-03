/*
Advent of Code 2022: Day 3 Rucksack Reorganization

https://adventofcode.com/2022/day/3
*/

use std::collections::HashMap;
use std::collections::HashSet;
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

fn init_map_priorities() -> HashMap<char, usize> {
    let mut priorities: HashMap<char, usize> = HashMap::<char, usize>::new();
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".chars().collect::<Vec<char>>();
    // lower letter in map
    for lower in letters.iter().enumerate() {
        priorities.insert(*lower.1, lower.0 + 1 as usize);
    }
    // upper letter in map
    for lower in letters.iter().enumerate() {
        let c: char = *lower.1;
        let uc = c.to_uppercase().collect::<Vec<char>>()[0];
        priorities.insert(uc, 26 + lower.0 + 1 as usize);
    }

    priorities
}

fn rucksack_part1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let priorities: HashMap<char, usize> = init_map_priorities();
    for line in lines {
        if !line.is_empty() {
            let length = line.len();
            let low = &line[0..length / 2];
            let high = &line[(length / 2)..length];
            let mut hash_low: HashSet<char> = HashSet::new();
            for c in low.chars() {
                hash_low.insert(c);
            }
            let mut hash_high: HashSet<char> = HashSet::new();
            for c in high.chars() {
                hash_high.insert(c);
            }
            let unique: Vec<&char> = hash_high.intersection(&hash_low).collect::<Vec<&char>>();
            result += *priorities.get(&*unique[0]).unwrap() as i32;
        }
    }

    result
}

fn chunk_to_set(chunk: &String) -> HashSet<char> {
    let mut set: HashSet<_> = HashSet::new();
    let chars: Vec<_> = chunk.chars().collect();
    for c in &chars {
        set.insert(*c);
    }

    set
}

fn unique_in_sets(a: HashSet<char>, b: HashSet<char>, c: HashSet<char>) -> Vec<char> {
    let single: HashSet<char> = &(&a & &b) & &c;
    let v: Vec<char> = single.into_iter().collect();

    v
}

fn rucksack_part2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let priorities: HashMap<char, usize> = init_map_priorities();
    for chunk in lines.chunks(3) {
        if chunk.len() > 1 {
            let set: HashSet<char> = chunk_to_set(&chunk[0]);
            let set2: HashSet<char> = chunk_to_set(&chunk[1]);
            let set3: HashSet<char> = chunk_to_set(&chunk[2]);
            let v: Vec<char> = unique_in_sets(set, set2, set3);
            result += *priorities.get(&v[0]).unwrap() as i32;
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
    result = rucksack_part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = rucksack_part2(&lines);
    println!("Result Part 2: {:?}", result);
}
