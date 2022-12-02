/*
Advent of Code 2022: Day 2 Rock Paper Scissors

https://adventofcode.com/2022/day/2
*/

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

const SCORE_LOST: i32 = 0;
const SCORE_DRAW: i32 = 3;
const SCORE_WIN:  i32 = 6;

fn readlines(filepath: &Path) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let binding = fs::read_to_string(filepath).unwrap();
    for line in binding.split("\n") {
        lines.push(String::from(line));
    }

    lines
}

fn rock_paper_round_score(input: String, scores: HashMap<&str, i32>) -> i32 {
    let result = *scores.get(&*input).unwrap() as i32;

    result
}

fn rock_paper_scissors_part1(lines: &Vec<String>) -> i32 {
    let mut total_score: i32 = 0;
    let scores = HashMap::from([
        ("A X", SCORE_DRAW + 1), // Rock  - Rock
        ("B Y", SCORE_DRAW + 2), // Paper - Paper
        ("C Z", SCORE_DRAW + 3), // Scissors - Scissors
        ("A Y", SCORE_WIN  + 2), // Rock - Paper
        ("A Z", SCORE_LOST + 3), // Rock - Scissors
        ("B X", SCORE_LOST + 1), // Paper - Rock
        ("B Z", SCORE_WIN  + 3), // Paper - Scissors
        ("C X", SCORE_WIN  + 1), // Scissors - Rock
        ("C Y", SCORE_LOST + 2), // Scissors - Paper
    ]);

    for line in lines {
        if !line.is_empty() {
            total_score += rock_paper_round_score(line.to_string(), scores.clone());
        }
    }

    total_score
}

fn rock_paper_scissors_part2(lines: &Vec<String>) -> i32 {
    let mut total_score: i32 = 0;
    let scores = HashMap::from([
        ("A X", SCORE_LOST + 3),
        ("A Y", SCORE_DRAW + 1),
        ("A Z", SCORE_WIN  + 2),
        ("B X", SCORE_LOST + 1),
        ("B Y", SCORE_DRAW + 2),
        ("B Z", SCORE_WIN  + 3),
        ("C X", SCORE_LOST + 2),
        ("C Y", SCORE_DRAW + 3),
        ("C Z", SCORE_WIN  + 1),
    ]);

    for line in lines {
        if !line.is_empty() {
            total_score += rock_paper_round_score(line.to_string(), scores.clone());
        }
    }

    total_score
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
    result = rock_paper_scissors_part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = rock_paper_scissors_part2(&lines);
    println!("Result Part 2: {:?}", result);
}
