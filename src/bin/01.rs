use aoc24::DATA_PATH;
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let data_path = [DATA_PATH, "01.txt"].iter().collect::<PathBuf>();
    let data =
        BufReader::new(File::open(data_path).expect("file containing task data should open"));
    let (mut left, mut right) = data
        .lines()
        .map(|line| {
            line.expect("each line should load correctly from the file")
                .split_once("   ")
                .map(|(left, right)| {
                    (
                        left.parse::<u32>().expect(
                            "the left column should contain a valid 32-bit unsigned integer",
                        ),
                        right.parse::<u32>().expect(
                            "the right column should contain a valid 32-bit unsigned integer",
                        ),
                    )
                })
                .expect("each line should contain three consecutive spaces")
        })
        .collect::<(Vec<_>, Vec<_>)>();

    left.sort_unstable();
    right.sort_unstable();

    let total_distance = left
        .iter()
        .zip(right.iter())
        .map(|(&left, &right)| left.abs_diff(right))
        .sum::<u32>();

    println!("total_distance = {total_distance}");

    let right_count = right
        .chunk_by(PartialEq::eq)
        .map(|chunk| (chunk[0], chunk.len() as u32))
        .collect::<HashMap<_, _>>();

    let similarity_score = left
        .into_iter()
        .map(|value| value * right_count.get(&value).unwrap_or(&0))
        .sum::<u32>();

    println!("similarity_score = {similarity_score}");
}
