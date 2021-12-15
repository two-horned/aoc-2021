use asos::reader::read_lines;

fn part2(){
}

fn part1(line: Vec<String>) -> usize {
    let mut counter_all = 0;
    println!("{}", line[0]);
    for each in line {
        let new: Vec<String> = each
            .split('|')
            .map(|e| e.to_string())
            .collect();
        let searched: Vec<String> = new[1]
            .split_whitespace()
            .map(|e| e.to_string())
            .collect();
        for e in searched {
            match e.chars().count() {
                2 => counter_all += 1,
                3 => counter_all += 1,
                4 => counter_all += 1,
                7 => counter_all += 1,
                _ => (),
            }
        }
    }
    counter_all
}

fn main() {
    let line: Vec<String> = read_lines("8");
    println!("part1: {}", part1(line));
}
