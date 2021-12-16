use asos::reader::read_lines;

fn part2(lines: &Vec<String>) -> usize {
    let mut sum = 0;
    for line in lines {
        let mut number = "".to_string();
        let mapper = map_signal(&line);
        let new: Vec<&str> = line
            .split('|')
            .map(|e| e.trim())
            .collect();
        let searched: Vec<&str> = new[1]
            .split_whitespace()
            .collect();
        for s in searched {
            for (i, m) in mapper.iter().enumerate() {
                if s.len() == m.len() && s.chars().all(|c| m.contains(c)) {
                    number += &i.to_string();
                }
            }
        }
        let n: usize = number.parse().unwrap();
        sum += n;
    }
    sum
}

fn map_signal(line: &String) -> [&str;10]  {
    let new: Vec<&str> = line
        .split('|')
        .map(|e| e.trim())
        .collect();
    let resolver: Vec<&str> = new[0]
        .split_whitespace()
        .collect();

    let one = resolver
        .iter()
        .find(|&s| s.len() == 2)
        .unwrap()
        .to_owned();
    let four = resolver
        .iter()
        .find(|&s| s.len() == 4)
        .unwrap()
        .to_owned();
    let seven = resolver
        .iter()
        .find(|&s| s.len() == 3)
        .unwrap()
        .to_owned();
    let eight = resolver
        .iter()
        .find(|&s| s.len() == 7)
        .unwrap()
        .to_owned();

    let three = resolver
        .iter()
        .find(|&s| s.len() == 5 && one.chars().all(|c| s.contains(c)))
        .unwrap()
        .to_owned();
    let nine = resolver
        .iter()
        .find(|&s| s.len() == 6 && four.chars().all(|c| s.contains(c)))
        .unwrap()
        .to_owned();
    let six = resolver
        .iter()
        .find(|&s| s.len() == 6 && !one.chars().all(|c| s.contains(c)))
        .unwrap()
        .to_owned();
    let zero = resolver
        .iter()
        .find(|&s| s.len() == 6 && s != &nine && s != &six)
        .unwrap()
        .to_owned();

    let five = resolver
        .iter()
        .find(|&s| s.len() == 5 && s != &three && s.chars().all(|c| six.contains(c)))
        .unwrap()
        .to_owned();
    let two = resolver
        .iter()
        .find(|&s| s.len() == 5 && s != &five && s != &three)
        .unwrap()
        .to_owned();
    [zero, one, two, three, four, five, six, seven, eight, nine]
}

fn part1(lines: &Vec<String>) -> usize {
    let mut counter_all = 0;
    for line in lines {
        let new: Vec<String> = line
            .split('|')
            .map(|e| e.to_string())
            .collect();
        let searched: Vec<String> = new[1]
            .split_whitespace()
            .map(|e| e.to_string())
            .collect();
        for e in searched {
            match e.chars().count() {
                2|3|4|7 => counter_all += 1,
                _ => (),
            }
        }
    }
    counter_all
}

fn main() {
    let line: Vec<String> = read_lines("8");
    println!("part1: {}", part1(&line));
    println!("part2: {}", part2(&line));
}
