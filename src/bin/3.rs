use std::time::Instant;
use asos::reader::read_matrix;
use asos::convertz::bin_vec_to_dec;

fn preferer(prefer: bool, matrix: &[Vec<u8>]) -> u32 {
    let zero = if prefer {0} else {1};
    let one = if prefer {1} else {0};
    let mut fake_matrix: Vec<Vec<u8>> = matrix.to_vec();
    for index in 0..matrix[0].len() {
        let times = fake_matrix
            .iter()
            .filter(|vec| vec[index] == 1)
            .count();
        if 2*times<fake_matrix.len() {
            fake_matrix.retain(|x| x[index]==zero);
        } else {
            fake_matrix.retain(|x| x[index]==one);
        }
        if fake_matrix.len()==1 {
            break;
        }
    }
    bin_vec_to_dec(&fake_matrix[0])
}

fn part2(matrix: &[Vec<u8>]) -> u32 {
    let gamma = preferer(true, matrix);
    let epsilon = preferer(false, matrix);
    gamma * epsilon
}

fn part1(matrix: &[Vec<u8>]) -> u32 {
    let mut gamma: Vec<u8> = vec![];
    for index in 0..matrix[0].len() {
        let times = matrix
            .iter()
            .filter(|vec| vec[index] == 1)
            .count();
        if 2*times < matrix.len() {
            gamma.push(0);
        } else {
            gamma.push(1);
        }
    }
    let a = bin_vec_to_dec(&gamma);
    let b = 2_u32.pow(matrix[0].len() as u32)-a-1;
    a * b
}

fn main() {
    let now = Instant::now();
    let matrix: Vec<Vec<u8>> = read_matrix("3");
    println!("part 1: {}", part1(&matrix));
    println!("part 2: {}", part2(&matrix));
    println!("Time: < {}ms", now.elapsed().as_millis() + 1);
}
