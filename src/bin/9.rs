use asos::{reader::read_matrix, xysys::Matrix2D};

fn part1(dingo: &Matrix2D) -> u8 {
    let mut counter = 0;
    for (x, each) in dingo.get_list().iter().enumerate() {
        for (y, e) in each.iter().enumerate() {
            let x = x as isize;
            let y = y as isize;
            let surround = dingo.get_surround(x, y);
            if surround.iter().all(|&c| c > Some(e.clone()) || c == None) {
                counter += e + 1;
            }
        }
    }
    counter
}

fn main() {
    let matrix: Vec<Vec<u8>> = read_matrix("example");
    let mut dingo: Matrix2D = Matrix2D::new();
    for each in matrix {
        dingo.push_line(each);
    }
    println!("part1: {}", part1(&dingo));
}
