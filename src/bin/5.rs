use std::{str::{FromStr, ParseBoolError}, collections::HashMap, cmp::Ordering};
use asos::reader::read_lines;

fn part2(lines: Vec<Destination>) -> usize {
    solution(lines)
}

fn part1(mut lines: Vec<Destination>) -> usize {
    lines.retain(|e| e.from.x == e.to.x || e.from.y == e.to.y);
    solution(lines)
}

fn solution(lines: Vec<Destination>) -> usize {
    let mut vec = Vec::new();
    let mut grid: Grid = Grid::new();
    for line in lines {
        let mut diffx = line.to.x - line.from.x;
        let killerx = match diffx.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
        let mut diffy = line.to.y - line.from.y;
        let killery = match diffy.cmp(&0) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        };
            vec.push((line.to.x-diffx, line.to.y-diffy));
        while diffx != 0 || diffy != 0 {
            diffx -= killerx;
            diffy -= killery;
            vec.push((line.to.x-diffx, line.to.y-diffy));
        }
    }
    grid.push_vec(vec);
    grid.count_overlap()
}

impl Grid {
    fn new() -> Grid {
        Grid {map: HashMap::new()}
    }

    fn push_vec(&mut self, vec: Vec<(i16,i16)>) {
        for v in vec {
            let counter = self.map.entry(v).or_insert(0);
            *counter += 1;
        }
    }
    fn count_overlap(self) -> usize {
        self.map
            .values()
            .into_iter()
            .filter(|&&v| v>1)
            .count()
    }
}

#[derive(Debug)]
struct Grid {
    map: HashMap<(i16, i16), u16>,
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


#[derive(Clone, Debug)]
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

#[derive(Clone, Copy, Debug)]
struct Coord {
    x: i16,
    y: i16,
}

fn main() {
    let lines: Vec<Destination> = read_lines("5");
    println!("{}", part1(lines.clone()));
    println!("{}", part2(lines));
}
