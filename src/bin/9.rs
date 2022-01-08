use std::time::Instant;
use std::collections::HashSet;
use asos::reader::read_matrix;
use asos::xysys::{Matrix2D, Point};

fn low_points(dingo: &Matrix2D) -> Vec<&Point> {
    let mut v = vec![];
    for point in dingo.list() {
        let surround = dingo.get_surround(point);
        if surround
            .iter()
            .all(|&c| if c.is_none() {
                true
            } else {
                c.unwrap().get() > point.get()
            }) {
            v.push(point);
        }
    }
    v
}

fn part2(dingo: &Matrix2D) -> usize {
    let mut basins = vec![];
    for low_point in low_points(&dingo) {
        let mut set = HashSet::new();
        let mut surround = vec![low_point];
        while let Some(point) = surround.pop() {
            set.insert(point);
            for s in dingo.get_surround(point) {
                if s.is_some() {
                    let s = s.unwrap();
                    if !set.contains(s) && s.get() != 9 {
                        surround.push(s);
                    }
                }
            }
        }
        basins.push(set.len());
    }
    basins.sort();
    basins.iter()
        .rev()
        .take(3)
        .product()
}


fn part1(dingo: &Matrix2D) -> u16 {
    low_points(&dingo)
        .iter()
        .map(|&x| (x.get() + 1) as u16)
        .sum()
}

fn main() {
    let now = Instant::now();
    let dingo: Matrix2D = Matrix2D::from(read_matrix("9"));
    println!("part1: {}", part1(&dingo));
    println!("part2: {}", part2(&dingo));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
