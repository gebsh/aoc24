use aoc24::DATA_PATH;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::PathBuf,
};

fn main() {
    let data_path = [DATA_PATH, "04.txt"].iter().collect::<PathBuf>();
    let grid = {
        let data =
            BufReader::new(File::open(data_path).expect("file containing task data should open"));
        let mut width = 0;
        let mut height = 0;
        let mut squares = Vec::new();

        for line in data.lines() {
            let line = line.expect("each line should load correctly from the file");
            width = line.len();
            height += 1;

            squares.extend(line.bytes().map(|b| {
                Letter::new(b)
                    .expect("the wordsearch puzzle should contain only X, M, A, and S letters")
            }));
        }

        Grid {
            width,
            height,
            squares,
        }
    };

    let xmas_count = count_xmas(&grid);

    println!("xmas_count = {xmas_count}");

    let x_mas_count = count_x_mas(&grid);

    println!("x_mas_count = {x_mas_count}");
}

const XMAS_LEN: usize = 4;

fn count_xmas(grid: &Grid) -> usize {
    let mut count: usize = 0;

    for i in 0..grid.squares.len() {
        if grid.squares[i] == Letter::X {
            for dir in get_directions(grid, i) {
                if is_xmas(grid, i, dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn count_x_mas(grid: &Grid) -> usize {
    let mut count: usize = 0;

    for i in 0..grid.squares.len() {
        let x = i % grid.width;
        let y = i / grid.width;

        if x > 0
            && x < grid.width - 1
            && y > 0
            && y < grid.height - 1
            && grid.squares[i] == Letter::A
            && is_x_mas(grid, i)
        {
            count += 1;
        }
    }

    count
}

struct Grid {
    width: usize,
    height: usize,
    squares: Vec<Letter>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Letter {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy)]
enum Direction {
    NorthWest,
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
}

impl Grid {
    fn translate(&self, i: usize, dir: Direction, len: usize) -> usize {
        let dx = dir.dx();
        let dy = dir.dy();
        let mut idx = i;

        if dx >= 0 {
            idx += len * (dx as usize);
        } else {
            idx -= len * dx.unsigned_abs();
        }

        if dy >= 0 {
            idx += len * (dy as usize) * self.width;
        } else {
            idx -= len * dy.unsigned_abs() * self.width;
        }

        idx
    }
}

impl Letter {
    fn new(b: u8) -> Option<Letter> {
        match b {
            b'X' => Some(Letter::X),
            b'M' => Some(Letter::M),
            b'A' => Some(Letter::A),
            b'S' => Some(Letter::S),
            _ => None,
        }
    }

    fn next(self) -> Option<Letter> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
        }
    }
}

impl Direction {
    fn dx(self) -> isize {
        match self {
            Direction::West | Direction::NorthWest | Direction::SouthWest => -1,
            Direction::North | Direction::South => 0,
            Direction::East | Direction::NorthEast | Direction::SouthEast => 1,
        }
    }

    fn dy(self) -> isize {
        match self {
            Direction::North | Direction::NorthWest | Direction::NorthEast => -1,
            Direction::West | Direction::East => 0,
            Direction::South | Direction::SouthWest | Direction::SouthEast => 1,
        }
    }
}

fn get_directions(grid: &Grid, i: usize) -> impl Iterator<Item = Direction> {
    let width = grid.width;
    let height = grid.height;
    let x = i % width;
    let y = i / width;
    let left_pad = x >= XMAS_LEN - 1;
    let right_pad = x <= width - XMAS_LEN;
    let top_pad = y >= XMAS_LEN - 1;
    let bot_pad = y <= height - XMAS_LEN;
    let mut dirs: [Option<Direction>; 8] = [None; 8];
    let mut i = 0;
    let mut add_dir = |dir: Direction| {
        dirs[i] = Some(dir);
        i += 1;
    };

    if left_pad && top_pad {
        add_dir(Direction::NorthWest);
    }

    if top_pad {
        add_dir(Direction::North);
    }

    if right_pad && top_pad {
        add_dir(Direction::NorthEast);
    }

    if right_pad {
        add_dir(Direction::East);
    }

    if right_pad && bot_pad {
        add_dir(Direction::SouthEast);
    }

    if bot_pad {
        add_dir(Direction::South);
    }

    if left_pad && bot_pad {
        add_dir(Direction::SouthWest);
    }

    if left_pad {
        add_dir(Direction::West);
    }

    dirs.into_iter().flatten()
}

fn is_xmas(grid: &Grid, x_idx: usize, dir: Direction) -> bool {
    let mut prev_letter: Letter = Letter::X;

    for i in 1..XMAS_LEN {
        let idx = grid.translate(x_idx, dir, i);
        let curr_letter = grid.squares[idx];
        let next_letter = prev_letter
            .next()
            .expect("previous letter should not be the last ");

        if curr_letter == next_letter {
            prev_letter = curr_letter
        } else {
            return false;
        }
    }

    true
}

fn is_x_mas(grid: &Grid, a_idx: usize) -> bool {
    let nw_letter = grid.squares[grid.translate(a_idx, Direction::NorthWest, 1)];
    let ne_letter = grid.squares[grid.translate(a_idx, Direction::NorthEast, 1)];
    let se_letter = grid.squares[grid.translate(a_idx, Direction::SouthEast, 1)];
    let sw_letter = grid.squares[grid.translate(a_idx, Direction::SouthWest, 1)];

    is_ms_pair(nw_letter, se_letter) && is_ms_pair(ne_letter, sw_letter)
}

fn is_ms_pair(first: Letter, second: Letter) -> bool {
    matches!(
        (first, second),
        (Letter::M, Letter::S) | (Letter::S, Letter::M)
    )
}
