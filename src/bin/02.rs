use aoc24::DATA_PATH;
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let data_path = [DATA_PATH, "02.txt"].iter().collect::<PathBuf>();
    let data =
        BufReader::new(File::open(data_path).expect("file containing task data should open"));
    let reports = data
        .lines()
        .map(|line| {
            line.expect("each line should load correctly from the file")
                .split(' ')
                .map(|value| {
                    value
                        .parse::<u32>()
                        .expect("each number in a report should be a valid 32-bit unsigned integer")
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_reports_count = reports
        .iter()
        .filter(|report| is_safe_report(report))
        .count();

    println!("safe_reports_count = {safe_reports_count}");

    let safe_damped_reports_count = reports
        .iter()
        .filter(|report| is_safe_damped_report(report))
        .count();

    println!("safe_damped_reports_count = {safe_damped_reports_count}");
}

const MAX_DIFF: u32 = 3;

fn is_safe_report(mut report: &[u32]) -> bool {
    match report.len() {
        0 => false,
        1 => true,
        _ => {
            let mut ord: Option<Ordering> = None;

            while let &[first, second, ..] = report {
                ord = match first.cmp(&second) {
                    Ordering::Equal => return false, // Not strictly monotonic.
                    curr_ord => match ord {
                        Some(o) if o == curr_ord => Some(o),
                        Some(_) => return false, // Not monotonic.
                        None => Some(curr_ord),
                    },
                };

                if first.abs_diff(second) > MAX_DIFF {
                    return false;
                }

                report = &report[1..];
            }

            true
        }
    }
}

fn is_safe_damped_report(report: &[u32]) -> bool {
    let buf_len = report.len() - 1;
    let mut buf: Vec<u32> = vec![0; buf_len];

    for i in 0..report.len() {
        let (first_part, second_part) = report.split_at(i);

        buf[0..i].copy_from_slice(first_part);
        buf[i..buf_len].copy_from_slice(&second_part[1..]);

        if is_safe_report(&buf) {
            return true;
        }
    }

    false
}
