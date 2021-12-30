use std::str::{FromStr, ParseBoolError};
use asos::reader::read_lines;

fn part2(lines: &Vec<Direction>) -> i32 {
    let mut forward = 0;
    let mut down = 0;
    let mut aim = 0;
    for line in lines {
        match line {
            Direction::Forward(value) => {
                forward += value;
                down += value * aim;
            },
            Direction::Down(value) => aim += value,
        }
    }
    forward * down
}

fn part1(lines: &Vec<Direction>) -> i32 {
    let mut forward = 0;
    let mut down = 0;
    for line in lines {
        match line {
            Direction::Forward(value) => forward += value,
            Direction::Down(value) => down += value,
        }
    }
    forward * down
}

impl FromStr for Direction {
    type Err = ParseBoolError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let change: Vec<String> = input
            .split_whitespace()
            .map(ToString::to_string)
            .collect();

        let direction: &str = &change[0];
        let value = change[1].parse().unwrap();

        Ok(match direction {
            "forward" => Self::Forward(value),
            "down" => Self::Down(value),
            "up" => Self::Down(-value),
            _ => unreachable!(),
        })
    }
}

enum Direction {
    Forward(i32),
    Down(i32),
}

fn main() {
    let lines: Vec<Direction> = read_lines("2");
    println!("part1{} {}", ':', part1(&lines));
    println!("part2{} {}", ':', part2(&lines));
}
