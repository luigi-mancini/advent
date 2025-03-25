mod utils;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

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

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Maze {
    grid: Grid,
    score_grid: ScoreGrid,
    start: Coordinates,
    end: Coordinates,
}

impl Maze {
    fn new(grid: Grid) -> Self {
        let start = Coordinates::new(grid.len() - 2, 1);
        let end = Coordinates::new(1, grid[0].len() - 2);
        let y = grid.len();
        let x = grid[0].len();

        Maze {
            grid,
            score_grid: vec![vec![None; x]; y],
            start,
            end,
        }
    }

    fn update_score_grid_entry(&mut self, y: usize, x: usize, score: usize) -> bool {
        match self.score_grid[y][x] {
            Some(val) => {
                if score < val {
                    self.score_grid[y][x] = Some(score);
                    true
                } else {
                    false
                }
            }
            None => {
                self.score_grid[y][x] = Some(score);
                true
            }
        }
    }

    fn create_score_grid(&mut self) -> Result<(), String> {
        let mut curr: HashMap<Coordinates, Direction> = HashMap::new();
        self.score_grid[self.start.y][self.start.x] = Some(0);
        curr.insert(self.start, Direction::Right);

        while !curr.is_empty() {
            let mut tmp: HashMap<Coordinates, Direction> = HashMap::new();

            for (c, d) in curr.iter() {
                let curr_score = self.score_grid[c.y][c.x].ok_or("Unexpected Score found")?;

                if self.grid[c.y - 1][c.x] != '#' && *d != Direction::Down {
                    let score = curr_score + 1 + (*d != Direction::Up) as usize * 1000;
                    if self.update_score_grid_entry(c.y - 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y - 1, c.x), Direction::Up);
                    }
                }

                if self.grid[c.y + 1][c.x] != '#' && *d != Direction::Up {
                    let score = curr_score + 1 + (*d != Direction::Down) as usize * 1000;
                    if self.update_score_grid_entry(c.y + 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y + 1, c.x), Direction::Down);
                    }
                }

                if self.grid[c.y][c.x - 1] != '#' && *d != Direction::Right {
                    let score = curr_score + 1 + (*d != Direction::Left) as usize * 1000;
                    if self.update_score_grid_entry(c.y, c.x - 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x - 1), Direction::Left);
                    }
                }

                if self.grid[c.y][c.x + 1] != '#' && *d != Direction::Left {
                    let score = curr_score + 1 + (*d != Direction::Right) as usize * 1000;
                    if self.update_score_grid_entry(c.y, c.x + 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x + 1), Direction::Right);
                    }
                }
            }

            curr = tmp;
        }

        Ok(())
    }
}

fn read_input(path: &str) -> io::Result<Grid> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let grid: Grid = reader
        .lines()
        .map(|x| match x {
            Ok(val) => Ok(val.chars().collect::<Vec<char>>()),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(grid)
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let grid = read_input("test.txt")?;
    utils::print_grid(&grid);

    let mut maze = Maze::new(grid);

    let _ = maze.create_score_grid();
    utils::print_grid_opt(&maze.score_grid);

    let score = maze.score_grid[maze.end.y][maze.end.x]
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Final score not found"))?;

    let end = start.elapsed();
    println!("Total Score {:?} in {:?}", score, end);

    Ok(())
}
