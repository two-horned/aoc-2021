use asos::reader::read_first_line;

fn part2(sorted_line: Vec<u32>) -> u32 {
    let max = sorted_line.last().unwrap().to_owned();
    let min = sorted_line.first().unwrap().to_owned();
    let mut vec = Vec::new();
    for pos in min..= max + 1 {
        let mut fuel = 0;
        for l in &sorted_line {
            let fake = if pos < l.to_owned() { l.to_owned() - pos } else { pos - l.to_owned() };
            fuel += fake * (fake + 1 ) / 2;
        }
        vec.push(fuel);
    }
    vec.sort();
    vec.first().unwrap().to_owned()
}

fn part1(sorted_line: Vec<u32>) -> u32 {
    let max = sorted_line.last().unwrap().to_owned();
    let min = sorted_line.first().unwrap().to_owned();
    let mut vec = Vec::new();
    for pos in min..= max + 1 {
        let mut fuel = 0;
        for l in &sorted_line {
            fuel += if pos < l.to_owned() { l.to_owned() - pos } else { pos - l.to_owned() };
        }
        vec.push(fuel);
    }
    vec.sort();
    vec.first().unwrap().to_owned()
}

fn main() {
    let mut line: Vec<u32> = read_first_line("7");
    line.sort();
    println!("part1: {}", part1(line.clone()));
    println!("part2: {}", part2(line));
}
