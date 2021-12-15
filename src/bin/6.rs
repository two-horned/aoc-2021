use std::collections::HashMap;
use asos::reader::read_first_line;

fn solution(vec: Vec<u8>, days: u16) -> usize {
    let mut ocean: Ocean = Ocean::new();
    for v in vec {
            let counter = ocean.fishmap.entry(v).or_insert(0);
        *counter += 1;
    }
    ocean.day_pass(days);
    ocean.count_fish()
}

impl Ocean { fn new() -> Ocean {
        Ocean {fishmap: HashMap::new()}
    }

    fn day_pass(&mut self, days: u16) {
        let mut now = 0;
        while now < days {
            let mut new_fishmap = HashMap::new();
            for fish in self.fishmap.clone() {
                if fish.0 == 0 {
                    let value = new_fishmap.entry(6).or_insert(0);
                    *value += fish.1;
                    new_fishmap.insert(8, fish.1);
                } else {
                    let value = new_fishmap.entry(fish.0 - 1).or_insert(0);
                    *value += fish.1;
                }
            }
            self.fishmap = new_fishmap;
            now += 1;
        }
    }

    fn count_fish(&self) -> usize {
        let mut all = 0;
        for fish in &self.fishmap {
            all += fish.1;
        }
        all
    }
}

struct Ocean {
    fishmap: HashMap<u8, usize>,
}

fn main() {
    let line: Vec<u8> = read_first_line("6");
    println!("part1: {}", solution(line.clone(), 80));
    println!("part2: {}", solution(line.clone(), 256));
}
