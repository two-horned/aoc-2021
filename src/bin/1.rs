use std::time::Instant;
use asos::reader::read_lines;

fn part2(lines: &Vec<u16>) -> u16 {
    let mut counter = 0;
    let mut current: u16 = lines.iter().take(3).sum();
    for line in lines.windows(3).skip(1) {
        if current < line.iter().sum() {
            counter += 1;
        }
        current = line.iter().sum();
    }
    counter
}

fn part1(lines: &Vec<u16>) -> u16 {
    let mut counter = 0;
    for line in lines.windows(2) {
        if line[0] < line[1] {
        counter += 1;
        }
    }
    counter
}

fn main() {
    let now = Instant::now();
    let lines = read_lines("1");
    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
