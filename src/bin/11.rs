use asos::reader::read_matrix;

fn part2(lines: &[Vec<u8>]) -> u16 {
    let mut counter = 0;
    counter
}

fn part1(matrix: &[Vec<u8>]) -> u16 {
    let mut matrix = matrix.clone();
    let mut day = 0;
    while day < 4 {
        let mut new_matrix = vec![];
        for line in matrix {
            let new_line: Vec<u8> = line
                .iter()
                .map(|&e| if e < 9 {e + 1} else { 0 })
                .collect();
            new_matrix.push(new_line);
        }
        println!("{:#?}", new_matrix);
        day += 1;
    }
    0
}

fn main() {
    let matrix: Vec<Vec<u8>> = read_matrix("example");
    println!("part1{} {}", ':', part1(&matrix));
    println!("part2{} {}", ':', part2(&matrix));
}
