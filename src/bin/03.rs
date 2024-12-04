use aoc24::DATA_PATH;
use std::{fs, path::PathBuf};

const DO_PATTERN: &str = "do()";
const DONT_PATTERN: &str = "don't()";
const MUL_START_PATTERN: &str = "mul(";

enum Instr {
    Do(bool),
    Mul(u32),
}

fn main() {
    let data_path = [DATA_PATH, "03.txt"].iter().collect::<PathBuf>();
    let data = fs::read_to_string(data_path)
        .expect("file containing task data should be read to a string");

    let mul_result = data
        .match_indices(MUL_START_PATTERN)
        .filter_map(|(i, _)| parse_mul(&data[i..]))
        .sum::<u32>();

    println!("mul_result = {mul_result}");

    let (_, mul_with_do_result) = data
        .match_indices(['d', 'm'])
        .filter_map(|(i, ch)| match ch {
            "d" => parse_do(&data[i..]).map(Instr::Do),
            "m" => parse_mul(&data[i..]).map(Instr::Mul),
            _ => None,
        })
        .fold((true, 0), |(enabled, sum), instr| match instr {
            Instr::Do(enable) => (enable, sum),
            Instr::Mul(val) if enabled => (enabled, sum + val),
            Instr::Mul(_) => (enabled, sum),
        });

    println!("mul_with_do_result = {mul_with_do_result}");
}

fn parse_do(s: &str) -> Option<bool> {
    match s.starts_with(DO_PATTERN) {
        true => Some(true),
        false => match s.starts_with(DONT_PATTERN) {
            true => Some(false),
            false => None,
        },
    }
}

fn parse_mul(s: &str) -> Option<u32> {
    let s = s.strip_prefix(MUL_START_PATTERN)?;
    let mut char_indices = s.char_indices();
    let mut find_idx = |pat_ch: char| -> Option<usize> {
        for (i, ch) in char_indices.by_ref() {
            match ch {
                '0'..='9' => {}
                _ if ch == pat_ch => return Some(i),
                _ => return None,
            }
        }

        None
    };
    let comma_idx = find_idx(',')?;
    let close_paren_idx = find_idx(')')?;
    let x = s[..comma_idx]
        .parse::<u32>()
        .expect("the multiplicand should be a valid 32-bit integer");
    let y = s[comma_idx + 1..close_paren_idx]
        .parse::<u32>()
        .expect("the multiplier should be a valid 32-bit integer");

    Some(x * y)
}
