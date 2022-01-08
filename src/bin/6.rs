use std::time::Instant;
use std::collections::BTreeMap;
use asos::reader::read_comma_line;

fn solution(vec: &Vec<u8>, days: u16) -> u64 {
    let mut ocean: Ocean = Ocean::new();
    for v in vec {
        *ocean.fishmap.entry(*v).or_insert(0) += 1;
    }
    ocean.day_pass(days);
    ocean.count_fish()
}

impl Ocean { fn new() -> Ocean {
        Ocean {fishmap: BTreeMap::new()}
    }

    fn day_pass(&mut self, days: u16) {
        let mut now = 0;
        while now < days {
        let mut new_fishmap = BTreeMap::new();
            for fish in &self.fishmap {
                if *fish.0 == 0 {
                    *new_fishmap.entry(6).or_insert(0) += fish.1;
                    new_fishmap.insert(8, *fish.1);
                } else {
                    *new_fishmap.entry(fish.0 - 1).or_insert(0) += fish.1;
                }
            }
            self.fishmap = new_fishmap;
            now += 1;
        }
    }

    fn count_fish(&self) -> u64 {
        let mut all = 0;
        for fish in &self.fishmap {
            all += fish.1;
        }
        all
    }
}

struct Ocean {
    fishmap: BTreeMap<u8, u64>,
}

fn main() {
    let now = Instant::now();
    let line: Vec<u8> = read_comma_line("6");
    println!("part1: {}", solution(&line, 80));
    println!("part2: {}", solution(&line, 256));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
