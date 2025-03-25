mod utils;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

type Grid = Vec<Vec<char>>;
type ScoreGrid = Vec<Vec<Option<usize>>>;
type CoordSet = HashSet<Coordinates>;
type Path = Vec<Coordinates>;
type AllPaths = Vec<Path>;

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

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Link {
    from: Coordinates,
    to: Coordinates,
}

impl Link {
    fn new(from: Coordinates, to: Coordinates) -> Self {
        Link { from, to }
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
    score: usize,
    all_paths: AllPaths,
    visited: HashSet<Link>,
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
            score: 0,
            all_paths: Vec::new(),
            visited: HashSet::new(),
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

                let mut update_curr_score = false;

                if self.grid[c.y - 1][c.x] != '#' && *d != Direction::Down {
                    let score = curr_score
                        + 1
                        + if *d != Direction::Up {
                            update_curr_score = true;
                            1000
                        } else {
                            0
                        };

                    if self.update_score_grid_entry(c.y - 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y - 1, c.x), Direction::Up);
                    }
                }

                if self.grid[c.y + 1][c.x] != '#' && *d != Direction::Up {
                    let score = curr_score
                        + 1
                        + if *d != Direction::Down {
                            update_curr_score = true;
                            1000
                        } else {
                            0
                        };
                    if self.update_score_grid_entry(c.y + 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y + 1, c.x), Direction::Down);
                    }
                }

                if self.grid[c.y][c.x - 1] != '#' && *d != Direction::Right {
                    let score = curr_score
                        + 1
                        + if *d != Direction::Left {
                            update_curr_score = true;
                            1000
                        } else {
                            0
                        };

                    if self.update_score_grid_entry(c.y, c.x - 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x - 1), Direction::Left);
                    }
                }

                if self.grid[c.y][c.x + 1] != '#' && *d != Direction::Left {
                    let score = curr_score
                        + 1
                        + if *d != Direction::Right {
                            update_curr_score = true;
                            1000
                        } else {
                            0
                        };

                    if self.update_score_grid_entry(c.y, c.x + 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x + 1), Direction::Right);
                    }
                }

                if update_curr_score && self.end.y != c.y && self.end.x != c.x {
                    self.score_grid[c.y][c.x] = Some(curr_score + 1000)
                }
            }

            curr = tmp;
        }

        self.score = self.score_grid[self.end.y][self.end.x].unwrap_or_default();

        Ok(())
    }

    fn find_shortess_paths(&mut self) {
        let mut path: Vec<Coordinates> = Vec::new();

        self.sp_impl(
            Link::new(self.start, Coordinates::new(self.start.y - 1, self.start.x)),
            1001,
            &mut path,
            Direction::Up,
        );

        self.sp_impl(
            Link::new(self.start, Coordinates::new(self.start.y, self.start.x + 1)),
            1,
            &mut path,
            Direction::Right,
        );
    }

    fn sp_impl(&mut self, link: Link, score: usize, path: &mut Vec<Coordinates>, dir: Direction) {
        if score > self.score
            || self.grid[link.to.y][link.to.x] == '#'
            || self.visited.contains(&link)
        {
            return;
        }

        path.push(link.to);
        if self.grid[link.to.y][link.to.x] == 'E' {
            self.all_paths.push(path.clone());
            path.pop();
            return;
        }

        self.visited.insert(link);

        if dir != Direction::Down {
            self.sp_impl(
                Link::new(link.to, Coordinates::new(link.to.y - 1, link.to.x)),
                score + 1 + (dir != Direction::Up) as usize * 1000,
                path,
                Direction::Up,
            );
        }
        if dir != Direction::Up {
            self.sp_impl(
                Link::new(link.to, Coordinates::new(link.to.y + 1, link.to.x)),
                score + 1 + (dir != Direction::Down) as usize * 1000,
                path,
                Direction::Down,
            );
        }

        if dir != Direction::Right {
            self.sp_impl(
                Link::new(link.to, Coordinates::new(link.to.y, link.to.x - 1)),
                score + 1 + (dir != Direction::Left) as usize * 1000,
                path,
                Direction::Left,
            );
        }

        if dir != Direction::Left {
            self.sp_impl(
                Link::new(link.to, Coordinates::new(link.to.y, link.to.x + 1)),
                score + 1 + (dir != Direction::Right) as usize * 1000,
                path,
                Direction::Right,
            );
        }

        path.pop();
        self.visited.remove(&link);
    }

    fn check_node(&self, y: usize, x: usize, val: usize, all: &mut CoordSet, next: &mut CoordSet) {
        if let Some(tmp) = self.score_grid[y][x] {
            if tmp % 100 == val {
                all.insert(Coordinates::new(y, x));
                next.insert(Coordinates::new(y, x));
            }
        }
    }

    fn find_all_nodes(&self) -> io::Result<usize> {
        let mut all_nodes: CoordSet = HashSet::new();
        let mut next_nodes: CoordSet = HashSet::new();

        let score = self.score_grid[self.end.y][self.end.x]
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Final score not found"))?;
        let mod_score = score % 100;

        all_nodes.insert(self.end);
        next_nodes.insert(self.end);

        for i in (0..mod_score).rev() {
            let mut tmp_nodes: CoordSet = HashSet::new();

            for n in &next_nodes {
                self.check_node(n.y - 1, n.x, i, &mut all_nodes, &mut tmp_nodes);
                self.check_node(n.y + 1, n.x, i, &mut all_nodes, &mut tmp_nodes);
                self.check_node(n.y, n.x - 1, i, &mut all_nodes, &mut tmp_nodes);
                self.check_node(n.y, n.x + 1, i, &mut all_nodes, &mut tmp_nodes);
            }
            next_nodes = tmp_nodes;
        }

        Ok(all_nodes.len())
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

    let grid = read_input("day16.txt")?;
    utils::print_grid(&grid);

    let mut maze = Maze::new(grid);

    let _ = maze.create_score_grid();
    utils::print_grid_opt(&maze.score_grid);

    let score = maze.score_grid[maze.end.y][maze.end.x]
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Final score not found"))?;

    //    let num_nodes = maze.find_all_nodes()?;

    maze.find_shortess_paths();

    let mut tmp: HashSet<Coordinates> = HashSet::new();

    for p in maze.all_paths {
        for n in p {
            tmp.insert(n);
        }
    }

    //println!("Paths {} {:?}", maze.all_paths.len(), maze.all_paths);

    let end = start.elapsed();
    println!(
        "Total Score {:?} num_nodes {} in {:?}",
        score,
        tmp.len(),
        end
    );

    Ok(())
}
