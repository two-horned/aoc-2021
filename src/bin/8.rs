use std::time::Instant;
use std::{str::{FromStr, ParseBoolError}, collections::BTreeMap};
use asos::reader::read_lines;

fn part2(lines: &Vec<Shiffre>) -> usize {
    let mut sum = 0;
    for line in lines {
        let mut number = String::from("");
        let mapper = line.map_signal();
        for s in &line.searcher {
            let k = *mapper.keys()
                .find(|t| s.len() == t.len() && s.chars().all(|c| t.contains(c)))
                .unwrap();
            number = number + &mapper[k];
        }
        let n: usize = number.parse().unwrap();
        sum += n;
    }
    sum
}

fn part1(lines: &Vec<Shiffre>) -> u16 {
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
            .map(|e| String::from(e))
            .collect();
        let resolver: Vec<String> = n[0]
            .split_whitespace()
            .map(|e|String::from(e))
            .collect();
        let searcher: Vec<String> = n[1]
            .split_whitespace()
            .map(|e| String::from(e))
            .collect();
        Ok(Shiffre {resolver: resolver, searcher: searcher})
    }
}

impl Shiffre {
    fn map_signal(&self) -> BTreeMap<&String, String>  {
        let mut map: BTreeMap<&String, String> = BTreeMap::new();
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
            map.insert(t, i.to_string());
        }
        map
    }
}

struct Shiffre {
    resolver: Vec<String>,
    searcher: Vec<String>,
}

fn main() {
    let now = Instant::now();
    let line: Vec<Shiffre> = read_lines("8");
    println!("part1: {}", part1(&line));
    println!("part2: {}", part2(&line));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
