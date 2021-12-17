use std::{str::{FromStr, ParseBoolError}, collections::BTreeMap};
use asos::reader::read_lines;

fn part2(lines: &Vec<Shiffre>) -> usize {
    let mut sum = 0;
    for line in lines {
        let mut number = "".to_string();
        let mapper = line.map_signal();
        for s in &line.searcher {
            let k = mapper.keys()
                .find(|t| s.len() == t.len() && s.chars().all(|c| t.contains(c)))
                .unwrap()
                .to_owned();
            number = number + &mapper.get(k).unwrap().to_string();
        }
        let n: usize = number.parse().unwrap();
        sum += n;
    }
    sum
}

fn part1(lines: &Vec<Shiffre>) -> usize {
    let mut counter = 0;
    for line in lines {
        for e in &line.searcher {
            match e.chars().count() {
                2|3|4|7 => counter += 1,
                _ => (),
            }
        }
    }
    counter
}

impl FromStr for Shiffre {
    type Err = ParseBoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n: Vec<String> = s
            .split('|')
            .map(|e| e.trim().to_string())
            .collect();
        let resolver: Vec<String> = n[0]
            .split_whitespace()
            .map(|e| e.to_string())
            .collect();
        let searcher: Vec<String> = n[1]
            .split_whitespace()
            .map(|e| e.to_string())
            .collect();
        Ok(Shiffre {resolver: resolver, searcher: searcher})
    }
}

impl Shiffre {
    fn map_signal(&self) -> BTreeMap<&String, usize>  {
        let mut map: BTreeMap<&String, usize> = BTreeMap::new();
        let one = self.resolver
            .iter()
            .find(|&s| s.len() == 2)
            .unwrap();
        let four = self.resolver
            .iter()
            .find(|&s| s.len() == 4)
            .unwrap();
        let seven = self.resolver
            .iter()
            .find(|&s| s.len() == 3)
            .unwrap();
        let eight = self.resolver
            .iter()
            .find(|&s| s.len() == 7)
            .unwrap();
        let three = self.resolver
            .iter()
            .find(|&s| s.len() == 5 && one.chars().all(|c| s.contains(c)))
            .unwrap();
        let nine = self.resolver
            .iter()
            .find(|&s| s.len() == 6 && four.chars().all(|c| s.contains(c)))
            .unwrap();
        let six = self.resolver
            .iter()
            .find(|&s| s.len() == 6 && !one.chars().all(|c| s.contains(c)))
            .unwrap();
        let zero = self.resolver
            .iter()
            .find(|s| s.len() == 6 && s != &nine && s != &six)
            .unwrap();
        let five = self.resolver
            .iter()
            .find(|s| s.len() == 5 && s != &three && s.chars().all(|c| six.contains(c)))
            .unwrap();
        let two = self.resolver
            .iter()
            .find(|s| s.len() == 5 && s != &five && s != &three)
            .unwrap();
        let a: [&String; 10] = [zero, one, two, three, four, five, six, seven, eight, nine];
        for (i, t) in a.iter().enumerate() {
            map.insert(t, i);
        }
        map
    }
}

struct Shiffre {
    resolver: Vec<String>,
    searcher: Vec<String>,
}

fn main() {
    let line: Vec<Shiffre> = read_lines("8");
    println!("part1: {}", part1(&line));
    println!("part2: {}", part2(&line));
}
