use aoc24::DATA_PATH;
use std::{
    cmp::Ordering,
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let data_path = [DATA_PATH, "05.txt"].iter().collect::<PathBuf>();
    let data =
        BufReader::new(File::open(data_path).expect("file containing task data should open"));
    let mut ordering_table = OrderingTable::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();
    let mut ordering_rules = true;

    for line in data.lines() {
        let line = line.expect("each line should load correctly from the file");

        if line.is_empty() {
            ordering_rules = false;
            continue;
        }

        if ordering_rules {
            parse_ordering_rule(&mut ordering_table, line);
        } else {
            parse_updates(&mut updates, line);
        }
    }

    let (correctly_ordered, incorrectly_ordered): (Vec<_>, Vec<_>) =
        updates.into_iter().partition(|update| {
            update
                .is_sorted_by(|&before, &after| ordering_table.get_ordering(before, after).is_lt())
        });

    let correct_middle_page_sum = correctly_ordered
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum::<u32>();

    println!("correct_middle_page_sum = {correct_middle_page_sum}");

    let incorrect_middle_page_sum = incorrectly_ordered
        .into_iter()
        .map(|mut update| {
            update.sort_unstable_by(|&before, &after| ordering_table.get_ordering(before, after));
            update[update.len() / 2]
        })
        .sum::<u32>();

    println!("incorrect_middle_page_sum = {incorrect_middle_page_sum}");
}

fn parse_ordering_rule(ordering_table: &mut OrderingTable, line: String) {
    let (before, after) = line
        .split_once('|')
        .expect("page ordering rules should be separated with |");
    let before = parse_page(before);
    let after = parse_page(after);

    ordering_table.insert_ordering(before, after);
}

fn parse_updates(pages: &mut Vec<Vec<u32>>, line: String) {
    pages.push(line.split(',').map(parse_page).collect())
}

fn parse_page(s: &str) -> u32 {
    s.parse::<u32>()
        .expect("each page should be a valid 32-bit unsigned integer")
}

struct OrderingTable {
    table: HashMap<u32, Vec<Ordering>>,
    cols: Vec<u32>,
}

impl OrderingTable {
    fn new() -> Self {
        Self {
            table: HashMap::new(),
            cols: Vec::new(),
        }
    }

    fn get_ordering(&self, before: u32, after: u32) -> Ordering {
        self.table[&before][self
            .get_col_idx(after)
            .expect("ordering rules should be added")]
    }

    fn insert_ordering(&mut self, before: u32, after: u32) {
        let before_col_idx = self.get_or_insert_col_idx(before);
        let after_col_idx = self.get_or_insert_col_idx(after);

        {
            let before_row = self.get_or_insert_row(before);
            before_row[after_col_idx] = Ordering::Less;
        }

        {
            let after_row = self.get_or_insert_row(after);
            after_row[before_col_idx] = Ordering::Greater;
        }
    }

    fn get_col_idx(&self, value: u32) -> Option<usize> {
        self.cols.iter().position(|&col| col == value)
    }

    fn get_or_insert_col_idx(&mut self, value: u32) -> usize {
        self.get_col_idx(value).unwrap_or_else(|| {
            let idx = self.cols.len();

            self.cols.push(value);

            for row in self.table.values_mut() {
                row.push(Ordering::Equal);
            }

            idx
        })
    }

    fn get_or_insert_row(&mut self, value: u32) -> &mut Vec<Ordering> {
        self.table
            .entry(value)
            .or_insert_with(|| vec![Ordering::Equal; self.cols.len()])
    }
}
