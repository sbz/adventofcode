/*
Advent of Code 2022: Day 5 Supply Stacks

https://adventofcode.com/2022/day/5
*/

extern crate regex;

use std::cmp;
use std::convert::TryInto;
use std::env;
use std::fmt;
use std::fs;
use std::path::Path;
use std::process;

use regex::Regex;

macro_rules! ntimes {
    ($n:expr, $body:block) => {
        for _ in 0..$n {
            $body
        }
    };
}

#[derive(Copy, Clone, Debug)]
struct Move(i32, i32, i32);

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "move {} from {} to {}", self.0, self.1, self.2)
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

fn max_number_crate_stacks(stacks: Vec<Vec<String>>) -> usize {
    let mut max: usize = 0;
    let number_of_stack: usize = stacks.len() - 1;
    for i in 1..=number_of_stack {
        max = cmp::max(stacks[i].len(), max);
    }

    max
}

fn draw_stacks(stacks: Vec<Vec<String>>) {
    let number_of_stack: usize = stacks.len() - 1; // do not count first index 0
    let max_value: usize = max_number_crate_stacks(stacks.clone());

    for i in 0..max_value {
        for j in 1..=number_of_stack {
            if i < stacks[j].len() {
                print!(" [{:?}]", stacks[j][i]);
            } else {
                print!(" [   ]");
            }
        }
        println!();
    }
    for j in 1..=number_of_stack {
        print!("   {}  ", j);
    }
    println!();
}

fn move_stacks(stacks: &mut Vec<Vec<String>>, m: Move, is_part2: bool) {
    let number: usize = m.0.try_into().unwrap();
    let from_index: usize = m.1.try_into().unwrap();
    let to_index: usize = m.2.try_into().unwrap();

    if number == 0 || stacks[from_index].len() == 0 {
        return;
    }

    if is_part2 {
        if number == 1 {
            let crate_moved: String = stacks[from_index].remove(0).to_string();
            stacks[to_index].insert(0, crate_moved);
            return;
        }

        let mut crates_to_move: Vec<String> = Vec::new();
        ntimes!(number, {
            let crate_moved: String = stacks[from_index].remove(0).to_string();
            crates_to_move.push(crate_moved);
        });

        crates_to_move.reverse();

        for keep_order_crate in crates_to_move {
            stacks[to_index].insert(0, keep_order_crate);
        }
    } else {
        ntimes!(number, {
            let crate_moved: String = stacks[from_index].remove(0).to_string();
            stacks[to_index].insert(0, crate_moved);
        });
    }
}

fn to_vec_string(str: &str) -> Vec<String> {
    let chars = str.chars();

    chars.map(|x| x.to_string()).collect::<Vec<String>>()
}

fn init_stacks(filename: &str) -> Vec<Vec<String>> {
    let mut stacks: Vec<Vec<String>> = Vec::new();

    if !filename.eq("input.txt") {
        stacks.push(Vec::new()); // not used, only for 1-indexed
        stacks.push(to_vec_string("NZ"));
        stacks.push(to_vec_string("DCM"));
        stacks.push(to_vec_string("P"));
    } else {
        stacks.push(Vec::new()); // not used, only for 1-indexed
        stacks.push(to_vec_string("QHCTNSVB"));
        stacks.push(to_vec_string("GBDW"));
        stacks.push(to_vec_string("BQSTRWF"));
        stacks.push(to_vec_string("NDJZSWGL"));
        stacks.push(to_vec_string("FVDPM"));
        stacks.push(to_vec_string("JWF"));
        stacks.push(to_vec_string("VJBQNL"));
        stacks.push(to_vec_string("NSQJCRTG"));
        stacks.push(to_vec_string("MDWCQSJ"));
    }

    stacks
}

fn init_moves(lines: &Vec<String>) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pattern = Regex::new(r"move (?P<number>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for line in lines {
        if !line.contains("move") {
            continue;
        }
        let groups = pattern.captures(line).unwrap();
        moves.push(Move(
            groups["number"].parse::<i32>().unwrap(),
            groups["from"].parse::<i32>().unwrap(),
            groups["to"].parse::<i32>().unwrap(),
        ));
    }

    moves
}

fn stack_part1(lines: &Vec<String>, filepath: &str) -> String {
    let mut result: String = String::from("");
    let is_part2: bool = false;
    let mut stacks: Vec<Vec<String>> = init_stacks(filepath);
    let moves: Vec<Move> = init_moves(&lines);

    for m in moves {
        //draw_stacks(stacks.clone());
        move_stacks(&mut stacks, m, is_part2);
        //println!("{}", m);
    }
    draw_stacks(stacks.clone());

    for i in 1..=stacks.len() - 1 {
        result.push_str(&stacks[i][0]);
    }

    result
}

fn stack_part2(lines: &Vec<String>, filepath: &str) -> String {
    let mut result: String = String::from("");
    let is_part2: bool = true;
    let mut stacks: Vec<Vec<String>> = init_stacks(filepath);
    let moves: Vec<Move> = init_moves(&lines);

    for m in moves {
        //draw_stacks(stacks.clone());
        move_stacks(&mut stacks, m, is_part2);
        //println!("{}", m);
    }
    draw_stacks(stacks.clone());

    for i in 1..=stacks.len() - 1 {
        result.push_str(&stacks[i][0]);
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
    let mut result: String;

    let lines = readlines(Path::new(filepath));
    result = stack_part1(&lines, filepath);
    println!("Result Part 1: {}", result);
    result = stack_part2(&lines, filepath);
    println!("Result Part 2: {}", result);
}
