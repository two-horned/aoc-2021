use std::path::PathBuf;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use std::io::{BufReader, BufRead};

pub fn read_lines<T: FromStr>(filename: &str) -> Vec<T>
    where
    <T as FromStr>::Err: Error,
{
    let path = PathBuf::from("data").join(filename);
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);

    buffer
        .lines()
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect()
}

pub fn read_matrix<T: FromStr>(filename: &str) -> Vec<Vec<T>>
where
    <T as FromStr>::Err: Error,
{
    let path = PathBuf::from("data").join(filename);
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);

    let mut matrix: Vec<Vec<T>> = vec![];
    for line in buffer.lines() {
        let little = line
            .unwrap()
            .chars()
            .map(|char| char.to_string().parse().unwrap())
            .collect();

        matrix.push(little);
    }
    matrix
}

pub fn read_comma_line<T: FromStr>(filename: &str) -> Vec<T>
    where
        <T as FromStr>::Err: Error,
{
    let path = PathBuf::from("data").join(filename);
    let file = File::open(path).unwrap();
    let buffer = BufReader::new(file);

    let lines: Vec<String> = buffer
        .lines()
        .map(|line| line.unwrap())
        .collect();

    lines[0]
        .split(',')
        .map(|char| char.parse().unwrap())
        .collect()
}
