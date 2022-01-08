use std::time::Instant;
use std::str::{FromStr, ParseBoolError};
use std::collections::HashMap;
use asos::reader::read_lines;

fn part1(lines: &Vec<Destination>) -> usize {
    let mut lines = lines.clone();
    lines.retain(|e| e.from.x == e.to.x || e.from.y == e.to.y);
    solution(&lines)
}

fn solution(lines: &Vec<Destination>) -> usize {
    let mut vec = vec![];
    let mut grid = HashMap::new();
    for line in lines {
        let mut diffx = line.to.x - line.from.x;
        let mut diffy = line.to.y - line.from.y;
        vec.push([line.from.x, line.from.y]);
        while diffx != 0 || diffy != 0 {
            diffx -= diffx.signum();
            diffy -= diffy.signum();
            vec.push([line.to.x-diffx, line.to.y-diffy]);
        }
    }
    for v in vec {
        *grid.entry(v).or_insert(0) += 1;
    }
    grid
        .iter()
        .filter(|(_, &v)| v>1)
        .count()
}

impl FromStr for Destination {
    type Err = ParseBoolError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let desti: Vec<Coord> = input
            .split(" -> ")
            .map(|s| s.parse().unwrap())
            .collect();
        Ok(Destination {from: desti[0], to: desti[1]})
    }
}

#[derive(Clone, Copy)]
struct Destination {
    from: Coord,
    to: Coord,
}

impl FromStr for Coord {
    type Err = ParseBoolError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coord: Vec<i16> = input
            .split(',')
            .map(|me| me.parse().unwrap())
            .collect();
        Ok(Coord {x: coord[0], y: coord[1],})
    }
}

#[derive(Clone, Copy)]
struct Coord {
    x: i16,
    y: i16,
}

fn main() {
    let now = Instant::now();
    let lines: Vec<Destination> = read_lines("5");
    println!("{}", part1(&lines));
    println!("{}", solution(&lines));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
