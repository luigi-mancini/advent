mod utils;

use anyhow::{anyhow, Error, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

type ByteList = Vec<Coordinates>;
type Grid = Vec<Vec<char>>;
type ScoreGrid = Vec<Vec<Option<usize>>>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Coordinates {
    y: usize,
    x: usize,
}

impl Coordinates {
    fn new(y: usize, x: usize) -> Self {
        Coordinates { y, x }
    }
}

#[derive(Debug)]
struct Maze {
    size: usize,
    grid: Grid,
    byte_list: ByteList,
    score_grid: ScoreGrid,
    start: Coordinates,
    end: Coordinates,
}

impl Maze {
    fn new(size: usize, byte_list: ByteList) -> Self {
        Maze {
            size,
            grid: vec![vec!['.'; size]; size],
            byte_list,
            score_grid: vec![vec![None; size]; size],
            start: Coordinates::new(0, 0),
            end: Coordinates::new(size - 1, size - 1),
        }
    }

    fn apply_bytes(&mut self, num: usize) {
        for i in 0..num {
            let x = self.byte_list[i].x;
            let y = self.byte_list[i].y;
            self.grid[y][x] = '#';
        }
    }

    fn update_score_grid_entry(&mut self, y: usize, x: usize, score: usize) -> bool {
        match self.score_grid[y][x] {
            Some(_val) => false,
            None => {
                self.score_grid[y][x] = Some(score);
                true
            }
        }
    }

    fn create_score_grid(&mut self) -> Result<()> {
        let mut curr: HashSet<Coordinates> = HashSet::new();
        self.score_grid[self.start.y][self.start.x] = Some(0);
        curr.insert(self.start);

        while !curr.is_empty() {
            let mut tmp: HashSet<Coordinates> = HashSet::new();

            for c in curr.iter() {
                let curr_score = (self.score_grid[c.y][c.x])
                    .ok_or_else(|| anyhow!("Unxpcted None value found in ScoreGrid"))?;

                if c.y != 0 && self.grid[c.y - 1][c.x] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y - 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y - 1, c.x));
                    }
                }

                if c.y < self.size - 1 && self.grid[c.y + 1][c.x] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y + 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y + 1, c.x));
                    }
                }

                if c.x != 0 && self.grid[c.y][c.x - 1] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y, c.x - 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x - 1));
                    }
                }

                if c.x < self.size - 1 && self.grid[c.y][c.x + 1] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y, c.x + 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x + 1));
                    }
                }
            }

            curr = tmp;
        }

        Ok(())
    }
}

fn read_input(path: &str) -> Result<ByteList> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let byte_list: ByteList = reader
        .lines()
        .filter(|line| match line {
            Ok(l) => !l.trim().is_empty(),
            Err(_) => true,
        })
        .map(|x| {
            let val = x?;

            let res: Vec<_> = val.split(',').collect();
            println!("{:?}", res);
            if res.len() != 2 {
                Err(Error::msg("Byte list entry does not contain 2 members"))
            } else {
                Ok(Coordinates::new(
                    res[1].parse::<usize>()?,
                    res[0].parse::<usize>()?,
                ))
            }
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(byte_list)
}

fn main() -> Result<()> {
    let start = Instant::now();

    let byte_list = read_input("day18.txt")?;
    let mut maze = Maze::new(71, byte_list);
    maze.apply_bytes(1024);

    utils::print_grid(&maze.grid);

    let _ = maze.create_score_grid();
    utils::print_grid_opt(&maze.score_grid);

    let score = maze.score_grid[maze.end.y][maze.end.x]
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Final score not found"))?;

    let end = start.elapsed();
    println!("Total Score {:?} in {:?}", score, end);

    Ok(())
}
