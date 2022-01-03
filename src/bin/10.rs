use std::collections::BTreeMap;
use asos::reader::read_matrix;

impl Syntax {
    fn new() -> Syntax {
    let mapping = BTreeMap::from([
        (')', '('),
        (']', '['),
        ('}', '{'),
        ('>', '<')
    ]);
    Syntax { mapping: mapping, opened: vec![] }
    }

    fn check_char(&mut self, c: char)  -> bool {
        if self.mapping.contains_key(&c) {
            if self.mapping.get(&c) == self.opened.last() {
                self.opened.pop();
            } else {
                return false;
            }
        } else {
            self.opened.push(c);
        }
        true
    }

    fn check_line(&mut self, line: &Vec<char>) -> Option<char> {
        for l in line {
            if !self.check_char(*l) {
                return Some(*l);
            }
        }
        None
    }

    fn clear_opened(&mut self) -> Vec<char> {
        let opened = self.opened.to_vec();
        self.opened = vec![];
        opened
    }
}

struct Syntax {
    mapping: BTreeMap<char, char>,
    opened: Vec<char>,
}

fn part2(lines: &[Vec<char>]) -> u32 {
    let bonus = BTreeMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores = vec![];
    let mut syntax = Syntax::new();
    for line in lines {
        let check = syntax.check_line(line);
        // Syntax Error  NOT Found
        if check == None {
            let mut score = 0;
            for o in syntax.opened.iter().rev() {
                score *= 5;
                score += bonus[o];
            }
            scores.push(score);
        }
        syntax.clear_opened();
    }
    scores.sort();
    scores[(scores.len() - 1) / 2 ]
    
}

fn part1(lines: &[Vec<char>]) -> u16 {
    let malus = BTreeMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut score = 0;
    let mut syntax = Syntax::new();
    for line in lines {
        let check = syntax.check_line(line);
        // Syntax Error Found
        if check != None {
            score += malus[&check.unwrap()];
        }
        syntax.clear_opened();
    }
    score
}

fn main() {
    let lines: Vec<Vec<char>> = read_matrix("10");
    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}
