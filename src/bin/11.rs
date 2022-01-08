use std::collections::BTreeSet;
use std::time::Instant;
use asos::reader::read_matrix;
use asos::xysys::Matrix2D;

fn day_pass(dingo: &mut Matrix2D) -> BTreeSet<[usize; 2]> {
    let mut to_flash: Vec<[usize; 2]> = dingo
        .list()
        .iter()
        .map(|&p| p.get_coord())
        .collect();
    let mut flashes = BTreeSet::new();
    while let Some([x, y]) = to_flash.pop() {
        if !flashes.contains(&[x, y]) {
            let point = dingo.get(x, y).unwrap();
            let mut value = point.get() + 1;
            if value > 9 {
                value = 0;
                println!("{:?}", [x, y]);
                flashes.insert([x, y]);
                let surround = dingo.get_all_surround(point);
                for s in surround {
                    if s.is_some() {
                        to_flash.push(s.unwrap().get_coord());
                    }
                }
            }
            dingo.set(x, y, value);
        }
    }
    flashes
}

fn part1(dingo: &Matrix2D) -> usize {
    let mut counter = 0;
    let mut dingo = dingo.clone();
    let mut day = 0;
    while day < 100 {
        counter += day_pass(&mut dingo).len();
        day += 1;
    }
    counter
}

fn main() {
    let now = Instant::now();
    let dingo: Matrix2D = Matrix2D::from(read_matrix("example"));
    println!("part1: {}", part1(&dingo));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
