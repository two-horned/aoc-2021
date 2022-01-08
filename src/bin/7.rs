use std::time::Instant;
use asos::reader::read_comma_line;

fn part2(sorted_line: &Vec<u32>, min: u32, max: u32) -> u32 {
    let mut vec = vec![];
    for pos in min..= max + 1 {
        let mut fuel = 0;
        for l in sorted_line {
            let diff = if pos < *l { *l - pos } else { pos - *l };
            fuel += diff * (diff + 1 ) / 2;
        }
        vec.push(fuel);
    }
    vec.sort();
    *vec.first().unwrap()
}

fn part1(sorted_line: &Vec<u32>, min: u32, max: u32) -> u32 {
    let mut vec = vec![];
    for pos in min..= max + 1 {
        let mut fuel = 0;
        for l in sorted_line {
            fuel += if pos < *l { *l - pos } else { pos - *l };
        }
        vec.push(fuel);
    }
    vec.sort();
    *vec.first().unwrap()
}

fn main() {
    let now = Instant::now();
    let mut line: Vec<u32> = read_comma_line("7");
    line.sort();
    let [min, max] = [*line.first().unwrap(), *line.last().unwrap()];
    println!("part1: {}", part1(&line, min, max));
    println!("part2: {}", part2(&line, min, max));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
