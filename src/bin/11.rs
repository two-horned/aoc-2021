use asos::reader::read_matrix;
use asos::xysys::{Matrix2D, Point};

fn part2(lines: &Matrix2D) -> u16 {
    let mut counter = 0;
    counter
}

fn part1(dingo: &Matrix2D) -> u16 {
    let mut day = 0;
    while day < 1 {
        let mut dingo = dingo.clone();
        for y in 0..=dingo.len() - 1 {
            for x in 0..=dingo.wth() - 1 {
                let point = dingo.get_point(x as isize, y as isize);
                println!("{:?}", point);
            }
        }
        day += 1;
    }
    0
}

fn main() {
    let dingo: Matrix2D = Matrix2D::from(read_matrix("example"));
    println!("part1{}", part1(&dingo));
    println!("part2{}", part2(&dingo));
}
