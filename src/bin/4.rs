use std::str::{FromStr, ParseBoolError};
use asos::reader::{read_comma_line, read_lines};

fn part2(bingo: &Bingo) -> u32 {
    let board_count = bingo.boards.len();
    let mut boards: Vec<Board> = bingo.boards.to_vec();
    let mut bingo_count = 0;
    for r in bingo.row.to_vec() {
        for board in boards.iter_mut().filter(|b| !b.check_bingo()) {
            board.mark_number(r);
            if board.check_bingo() {
                bingo_count += 1;
            }
            if bingo_count == board_count {
                return r * board.sum_unmarked();
            }
        }
    }
    0
}

fn part1(bingo: &Bingo) -> u32 {
    let mut bingo = bingo.clone();
    for r in bingo.row {
        for board in &mut bingo.boards {
            board.mark_number(r);
            if board.check_bingo() {
                return r * board.sum_unmarked();
            }
        }
    }
    0
}

fn to_bingo(filename: &str) -> Bingo {
    let mut boards: Vec<Board> = vec![];
    let mut lines: Vec<String> = read_lines(filename);
    lines.remove(0);
    lines.retain(|s| s!="");

    for chunk in lines.chunks(5) {
        let mut board = Board {lines: [[Number::new(); 5]; 5]};
        for index in 0..5 {
            let row: Vec<Number> = chunk[index]
            .split_whitespace()
            .map(|c| c.to_string().trim().parse().unwrap())
            .collect();
            for r in 0..5 {
                board.lines[index][r] = row[r];
            }
        }
        boards.push(board);
    }
    Bingo { row: read_comma_line(filename), boards: boards }
}

impl FromStr for Number {
    type Err = ParseBoolError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Number {marked: false,value: input.parse().unwrap()})
    }
}

impl Board {
    fn sum_unmarked(&self) -> u32 {
        let mut sum = 0;
        for line in &self.lines {
            for number in line {
                if !number.marked {
                    sum += number.value;
                }
            }
        }
        return sum
    }

    fn check_bingo(&self) -> bool {
        for (column, line) in self.lines.iter().enumerate() {
            let marked = line
                .iter()
                .filter(|&n| n.marked)
                .count();
            if marked == 5 {
                return true;
            }

            let marked = self.lines
                .iter()
                .filter(|&n| n[column].marked)
                .count();
                if marked == 5 {
                    return true;
                }
        }
        false
    }

    fn mark_number(&mut self, value: u32) {
        for row in &mut self.lines {
            for mut number in row {
                if number.value == value {
                    number.marked = true;
                }
            }
        }
    }
}

impl Number {
    fn new() -> Number {
        Number {marked: false, value: 0}
    }
}

#[derive(Clone, Copy, Debug)]
struct Number{
    marked: bool,
    value: u32,
}

#[derive(Clone, Debug)]
struct Board {
    lines: [[Number; 5]; 5],
}

#[derive(Clone, Debug)]
struct Bingo {
    row: Vec<u32>,
    boards: Vec<Board>,
}

fn main() {
    let bingo: Bingo = to_bingo("4");
    println!("part1: {}", part1(&bingo));
    println!("part2: {}", part2(&bingo));
}
