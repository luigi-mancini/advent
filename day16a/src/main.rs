mod utils;

use std::fs::File;
use std::io::{self, BufRead, Read};
use std::time::Instant;
use std::{thread, time::Duration};

type Grid = Vec<Vec<char>>;

#[derive(Debug)]
struct Coordinates {
    y: usize,
    x: usize,
}

impl Coordinates {
    fn new(y: usize, x: usize) -> Self {
        Coordinates { y, x }
    }
}

enum MoveOutcome {
    Wall,
    EmptySpace,
    Barrels(Vec<Coordinates>),
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

    fn find_shortest_path(&mut self) -> usize {
        0
    }
}
/*
    fn check_up(&self, y: usize, x: usize) -> MoveOutcome {
        if self.grid[y - 1][x] == '#' || self.grid[y - 1][x + 1] == '#' {
            return MoveOutcome::Wall;
        }

        if self.grid[y - 1][x] == '.' && self.grid[y - 1][x + 1] == '.' {
            return MoveOutcome::EmptySpace;
        }

        let mut v: Vec<Coordinates> = Vec::new();

        if self.grid[y - 1][x] == ']' {
            v.push(Coordinates::new(y - 1, x - 1));
        } else if self.grid[y - 1][x] == '[' {
            v.push(Coordinates::new(y - 1, x));
        }

        if self.grid[y - 1][x + 1] == '[' {
            v.push(Coordinates::new(y - 1, x + 1));
        }

        MoveOutcome::Barrels(v)
    }

    fn move_up(&mut self, y: &mut usize, x: usize) {
        if self.grid[*y - 1][x] == '#' {
            return;
        } else if self.grid[*y - 1][x] == '.' {
            self.grid[*y - 1][x] = '@';
            self.grid[*y][x] = '.';
            *y -= 1;
            return;
        }

        let start = {
            if self.grid[*y - 1][x] == '[' {
                Coordinates::new(*y - 1, x)
            } else if self.grid[*y - 1][x] == ']' {
                Coordinates::new(*y - 1, x - 1)
            } else {
                panic!("Unexpected character found in move_up");
            }
        };

        let mut all_barrels: Vec<Coordinates> = Vec::new();
        let mut curr_barrels: Vec<Coordinates> = vec![start];

        loop {
            let mut tmp: Vec<Coordinates> = Vec::new();
            for b in &curr_barrels {
                match self.check_up(b.y, b.x) {
                    MoveOutcome::Wall => {
                        return;
                    }
                    MoveOutcome::Barrels(mut v) => {
                        tmp.append(&mut v);
                    }
                    _ => (),
                }
            }

            all_barrels.append(&mut curr_barrels);

            if tmp.is_empty() {
                break;
            }

            curr_barrels = tmp;
        }

        eprintln!("MOVING BARRLES UP");
        eprintln!("{:?}", curr_barrels);
        for b in all_barrels.iter().rev() {
            self.grid[b.y - 1][b.x] = '[';
            self.grid[b.y - 1][b.x + 1] = ']';
            self.grid[b.y][b.x] = '.';
            self.grid[b.y][b.x + 1] = '.';
        }

        self.grid[*y - 1][x] = '@';
        self.grid[*y][x] = '.';
        *y -= 1;
    }

    fn check_down(&self, y: usize, x: usize) -> MoveOutcome {
        if self.grid[y + 1][x] == '#' || self.grid[y + 1][x + 1] == '#' {
            return MoveOutcome::Wall;
        }

        if self.grid[y + 1][x] == '.' && self.grid[y + 1][x + 1] == '.' {
            return MoveOutcome::EmptySpace;
        }

        let mut v: Vec<Coordinates> = Vec::new();

        if self.grid[y + 1][x] == ']' {
            v.push(Coordinates::new(y + 1, x - 1));
        } else if self.grid[y + 1][x] == '[' {
            v.push(Coordinates::new(y + 1, x));
        }

        if self.grid[y + 1][x + 1] == '[' {
            v.push(Coordinates::new(y + 1, x + 1));
        }

        MoveOutcome::Barrels(v)
    }

    fn move_down(&mut self, y: &mut usize, x: usize) {
        if self.grid[*y + 1][x] == '#' {
            return;
        } else if self.grid[*y + 1][x] == '.' {
            self.grid[*y + 1][x] = '@';
            self.grid[*y][x] = '.';
            *y += 1;
            return;
        }

        let start = {
            if self.grid[*y + 1][x] == '[' {
                Coordinates::new(*y + 1, x)
            } else if self.grid[*y + 1][x] == ']' {
                Coordinates::new(*y + 1, x - 1)
            } else {
                panic!("Unexpected character found in move_up");
            }
        };

        let mut all_barrels: Vec<Coordinates> = Vec::new();
        let mut curr_barrels: Vec<Coordinates> = vec![start];

        loop {
            let mut tmp: Vec<Coordinates> = Vec::new();
            for b in &curr_barrels {
                match self.check_down(b.y, b.x) {
                    MoveOutcome::Wall => {
                        return;
                    }
                    MoveOutcome::Barrels(mut v) => {
                        tmp.append(&mut v);
                    }
                    _ => (),
                }
            }

            all_barrels.append(&mut curr_barrels);

            if tmp.is_empty() {
                break;
            }

            curr_barrels = tmp;
        }

        for b in all_barrels.iter().rev() {
            self.grid[b.y + 1][b.x] = '[';
            self.grid[b.y + 1][b.x + 1] = ']';
            self.grid[b.y][b.x] = '.';
            self.grid[b.y][b.x + 1] = '.';
        }

        self.grid[*y + 1][x] = '@';
        self.grid[*y][x] = '.';
        *y += 1;
    }

    fn move_left(&mut self, y: usize, x: &mut usize) {
        for x1 in (0..*x).rev() {
            if self.grid[y][x1] == '#' {
                return;
            } else if self.grid[y][x1] == '.' {
                for x2 in (x1 + 1)..=*x {
                    self.grid[y][x2 - 1] = self.grid[y][x2];
                }
                self.grid[y][*x] = '.';
                *x -= 1;
                return;
            }
        }
    }

    fn move_right(&mut self, y: usize, x: &mut usize) {
        for x1 in (*x + 1)..self.grid[0].len() {
            if self.grid[y][x1] == '#' {
                return;
            } else if self.grid[y][x1] == '.' {
                for x2 in (*x..x1).rev() {
                    self.grid[y][x2 + 1] = self.grid[y][x2];
                }
                self.grid[y][*x] = '.';
                *x += 1;
                return;
            }
        }
    }

    fn get_next(&mut self) -> Option<char> {
        self.instr.pop()
    }

    fn execute(&mut self) {
        let (mut y, mut x) = self.find_start().expect("Starting point not found in grid");

        while let Some(i) = self.get_next() {
            match i {
                '^' => self.move_up(&mut y, x),
                'v' => self.move_down(&mut y, x),
                '<' => self.move_left(y, &mut x),
                '>' => self.move_right(y, &mut x),
                _ => (),
            }

            //println!("NEXT MOVE IS {}", i);
            //utils::print_grid(&self.grid);
            //  thread::sleep(Duration::from_millis(500))
        }
    }

    fn execute_stdin(&mut self) {
        let (mut y, mut x) = self.find_start().expect("Starting point not found in grid");

        loop {
            let mut buffer = [0; 1]; // Buffer to store a single byte
            io::stdin().read_exact(&mut buffer).unwrap(); // Read a single byte

            match buffer[0] as char {
                '^' => self.move_up(&mut y, x),
                'v' => self.move_down(&mut y, x),
                '<' => self.move_left(y, &mut x),
                '>' => self.move_right(y, &mut x),
                'q' => break,
                _ => (),
            }
            utils::print_grid(&self.grid);
        }
    }

    fn find_start(&mut self) -> Option<(usize, usize)> {
        for (y, v) in self.grid.iter().enumerate() {
            for (x, val) in v.iter().enumerate() {
                if *val == '@' {
                    return Some((y, x));
                }
            }
        }
        None
    }

    fn find_score(&mut self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, v)| {
                v.iter()
                    .enumerate()
                    .map(|(x, val)| {
                        if *val == '[' {
                            println!("Coord {} {}", y, x);
                            y * 100 + x
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}
*/
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
    //    let start = Instant::now();

    let grid = read_input("test.txt")?;
    utils::print_grid(&grid);

    let maze = Maze::new(grid);
    println!("Maze {:?}", maze);

    /*let score = sim.find_score();
    utils::print_grid(&sim.grid);

    let end = start.elapsed();
    println!("Total Score {:?} in {:?}", score, end);
    */
    Ok(())
}
