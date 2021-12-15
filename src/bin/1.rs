use asos::reader::read_lines;

fn part2(lines: &Vec<u16>) -> u16 {
    let mut counter = 0;
    let mut current: u16 = lines[..3].iter().sum();
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
    let lines = read_lines("1");
    println!("part1{} {}", ':', part1(&lines));
    println!("part2{} {}", ':', part2(&lines));
}