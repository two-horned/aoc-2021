use asos::reader::read_comma_line;

fn part2(sorted_line: &Vec<u32>) -> u32 {
    let max = *sorted_line.last().unwrap();
    let min = *sorted_line.first().unwrap();
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

fn part1(sorted_line: &Vec<u32>) -> u32 {
    let max = *sorted_line.last().unwrap();
    let min = *sorted_line.first().unwrap();
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
    let mut line: Vec<u32> = read_comma_line("7");
    line.sort();
    println!("part1: {}", part1(&line));
    println!("part2: {}", part2(&line));
}
