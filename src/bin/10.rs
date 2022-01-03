use asos::reader::{read_lines, read_matrix};

const opening: [char; 4] = ['(', '[', '{', '<'];
const ending: [char; 4] = [')', ']', '}', '>'];

fn part1(lines: &[Vec<char>]) -> u16 {
    let mut score = 0;
    for line in lines {
        let mut opened = vec![];
        for l in line {
            if opening.contains(l) {
                opened.push(l);
            } else if ending.contains(l) {
                match l {
                    ')' => 
            } else {
                panic!();
            }
        }
    }
    score
}

fn main() {
    let lines: Vec<Vec<char>> = read_matrix("example");
    println!("part1: {:#?}", part1(&lines));
}
