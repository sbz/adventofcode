/*
Advent of Code 2023: Day 2 Cube Conundrum

https://adventofcode.com/2023/day/2
*/

use std::env;
use std::fs;
use std::path::Path;
use std::process;

#[derive(Clone, Debug)]
struct GameSet {
    red: i32,
    green: i32,
    blue: i32,
}

impl GameSet {
    fn new(red: i32, green: i32, blue: i32) -> Self {
        GameSet {
            red: red,
            green: green,
            blue: blue,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: i32,
    cubes: Vec<GameSet>,
}

impl Game {
    fn new() -> Self {
        Game {
            id: 0,
            cubes: vec![
                GameSet {
                    red: 0,
                    green: 0,
                    blue: 0
                };
                3
            ],
        }
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

fn parse_gameset_from_line(line: &str) -> GameSet {
    /* 1 red, 2 green, 6 blue */
    let (mut r, mut g, mut b) = (0, 0, 0);
    let cubes: Vec<_> = line.split(",").collect();
    for cube in cubes {
        let item: Vec<_> = cube.trim().split(" ").collect();
        match item[1] {
            "red"   => r = item[0].parse::<i32>().unwrap(),
            "blue"  => b = item[0].parse::<i32>().unwrap(),
            "green" => g = item[0].parse::<i32>().unwrap(),
            _       => (),
        }
    }

    GameSet::new(r, g, b)
}

fn parse_game_from_line(line: &String) -> Game {
    /* Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green */
    let mut g = Game::new();
    let mut gs = Vec::<GameSet>::new();
    let part: Vec<_> = line.split(":").collect();
    if part.len() < 2 {
        panic!("malformed input no ':' found in {}", line);
    }
    let id: Vec<_> = part[0].split(" ").collect();
    let sets: Vec<_> = part[1].split(";").collect();
    for set_line in &sets {
        let gameset = parse_gameset_from_line(set_line);
        gs.push(gameset);
    }

    g.id = id[1].parse::<i32>().unwrap();
    g.cubes = gs;

    g
}

fn part1(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut games = Vec::<Game>::new();
    for line in lines {
        games.push(parse_game_from_line(&line));
    }
    let (max_red, max_green, max_blue) = (12, 13, 14);
    for game in games {
        let ok_red: bool = game.cubes.iter().map(|x| x.red <= max_red).all(|e| e);
        let ok_green: bool = game.cubes.iter().map(|x| x.green <= max_green).all(|e| e);
        let ok_blue: bool = game.cubes.iter().map(|x| x.blue <= max_blue).all(|e| e);
        if ok_red && ok_green && ok_blue {
            result += game.id;
        }
    }

    result
}

fn part2(lines: &Vec<String>) -> i32 {
    let mut result: i32 = 0;
    let mut games = Vec::<Game>::new();
    for line in lines {
        games.push(parse_game_from_line(&line));
    }
    for game in games {
        let nb_red: i32 = game.cubes.iter().map(|x| x.red).max().unwrap();
        let nb_green: i32 = game.cubes.iter().map(|x| x.green).max().unwrap();
        let nb_blue: i32 = game.cubes.iter().map(|x| x.blue).max().unwrap();
        result += nb_red * nb_green * nb_blue;
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
    result = part1(&lines);
    println!("Result Part 1: {:?}", result);
    result = part2(&lines);
    println!("Result Part 2: {:?}", result);
}
