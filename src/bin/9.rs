use std::collections::BTreeSet;
use asos::{reader::read_matrix, xysys::Matrix2D};

fn part2(dingo: &Matrix2D) -> usize {
    let lower_points = low_points(&dingo);
    let mut basins: Vec<usize> = vec![];
    for (low_x, low_y) in lower_points {
        let mut set: BTreeSet<(isize, isize)> = BTreeSet::new();
        let mut surround = vec![(low_x, low_y)];
        while let Some((x, y)) = surround.pop() {
            set.insert((x, y));
            let new_index: Vec<usize> = dingo.get_surround(x, y)
                .iter()
                .enumerate()
                .filter(|&(_, &x)| x != Some(9) && x != None)
                .map(|(e, &_)| e)
                .collect();
            for n in new_index {
                match n {
                    0 => surround.push((x-1, y)),
                    1 => surround.push((x, y+1)),
                    2 => surround.push((x+1, y)),
                    3 => surround.push((x, y-1)),
                    _ => unreachable!()
                }
            }
            surround.retain(|e| !set.contains(e));
        }
        basins.push(set.len());
    }
    basins.sort();
    basins
        .iter()
        .rev()
        .take(3)
        .product()
}

fn low_points(dingo: &Matrix2D) -> Vec<(isize, isize)> {
    let mut v = vec![];
    for (x, each) in dingo.get_list().iter().enumerate() {
        for (y, e) in each.iter().enumerate() {
            let x = x as isize;
            let y = y as isize;
            let surround = dingo.get_surround(x, y);
            if surround.iter().all(|&c| c > Some(*e) || c == None) {
                v.push((x, y));
            }
        }
    }
    v
}

fn part1(dingo: &Matrix2D) -> u16 {
    low_points(&dingo)
        .iter()
        .map(|&(x, y)| (dingo.get_val(x, y).unwrap() + 1) as u16)
        .sum()
}

fn main() {
    let matrix: Vec<Vec<u8>> = read_matrix("9");
    let mut dingo: Matrix2D = Matrix2D::new();
    for each in matrix {
        dingo.push_line(each);
    }
    println!("part1: {}", part1(&dingo));
    println!("part2: {}", part2(&dingo));
}
