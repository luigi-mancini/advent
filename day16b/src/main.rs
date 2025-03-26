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
    start: Coordinates,
    end: Coordinates,
}

impl Maze {
    fn new(grid: Grid) -> Self {
        let start = Coordinates::new(grid.len() - 2, 1);
        let end = Coordinates::new(1, grid[0].len() - 2);

        Maze { grid, start, end }
    }

    fn get_y(&self) -> usize {
        self.grid.len()
    }

    fn get_x(&self) -> usize {
        self.grid[0].len()
    }

    fn update_score_grid_entry(
        &mut self,
        y: usize,
        x: usize,
        score: usize,
        grid: &mut ScoreGrid,
    ) -> bool {
        match grid[y][x] {
            Some(val) => {
                if score < val {
                    grid[y][x] = Some(score);
                    true
                } else {
                    false
                }
            }
            None => {
                grid[y][x] = Some(score);
                true
            }
        }
    }

    fn create_score_grid(
        &mut self,
        grid: &mut ScoreGrid,
        start: Coordinates,
        dir: Direction,
    ) -> Result<(), String> {
        let mut curr: HashMap<Coordinates, Direction> = HashMap::new();

        grid[start.y][start.x] = Some(0);
        curr.insert(start, dir);

        while !curr.is_empty() {
            let mut tmp: HashMap<Coordinates, Direction> = HashMap::new();

            for (c, d) in curr.iter() {
                let curr_score = grid[c.y][c.x].ok_or("Unexpected Score found")?;

                if self.grid[c.y - 1][c.x] != '#' && *d != Direction::Down {
                    let score = curr_score + 1 + (*d != Direction::Up) as usize * 1000;
                    if self.update_score_grid_entry(c.y - 1, c.x, score, grid) {
                        tmp.insert(Coordinates::new(c.y - 1, c.x), Direction::Up);
                    }
                }

                if self.grid[c.y + 1][c.x] != '#' && *d != Direction::Up {
                    let score = curr_score + 1 + (*d != Direction::Down) as usize * 1000;
                    if self.update_score_grid_entry(c.y + 1, c.x, score, grid) {
                        tmp.insert(Coordinates::new(c.y + 1, c.x), Direction::Down);
                    }
                }

                if self.grid[c.y][c.x - 1] != '#' && *d != Direction::Right {
                    let score = curr_score + 1 + (*d != Direction::Left) as usize * 1000;
                    if self.update_score_grid_entry(c.y, c.x - 1, score, grid) {
                        tmp.insert(Coordinates::new(c.y, c.x - 1), Direction::Left);
                    }
                }

                if self.grid[c.y][c.x + 1] != '#' && *d != Direction::Left {
                    let score = curr_score + 1 + (*d != Direction::Right) as usize * 1000;
                    if self.update_score_grid_entry(c.y, c.x + 1, score, grid) {
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

fn count_nodes(score: usize, sg: &ScoreGrid, rsg: &ScoreGrid) -> usize {
    sg.iter()
        .zip(rsg.iter())
        .map(|(x, y)| {
            x.iter()
                .zip(y.iter())
                .map(|(val1, val2)| {
                    if let (Some(v1), Some(v2)) = (val1, val2) {
                        if v1 + v2 == score || v1 + v2 == score - 1000 {
                            1
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let grid = read_input("day16.txt")?;
    utils::print_grid(&grid);

    let mut maze = Maze::new(grid);

    let mut score_grid: ScoreGrid = vec![vec![None; maze.get_x()]; maze.get_y()];
    let _ = maze.create_score_grid(&mut score_grid, maze.start, Direction::Right);
    utils::print_grid_opt(&score_grid);

    let mut rev_score_grid: ScoreGrid = vec![vec![None; maze.get_x()]; maze.get_y()];
    let _ = maze.create_score_grid(&mut rev_score_grid, maze.end, Direction::Down);
    utils::print_grid_opt(&rev_score_grid);

    let score = score_grid[maze.end.y][maze.end.x]
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Final score not found"))?;

    let nodes = count_nodes(score, &score_grid, &rev_score_grid);

    let end = start.elapsed();
    println!("Total Score {:?} num_nodes {} in {:?}", score, nodes, end);

    Ok(())
}
