mod utils;

use std::fs::File;
use std::io::{self, BufRead, Read};
use std::time::Instant;
use std::{thread, time::Duration};

use regex::Regex;

type Grid = Vec<Vec<char>>;
type Instructions = Vec<char>;

#[derive(Debug, Copy, Clone)]
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

#[derive(Debug, Default)]
struct Simulation {
    grid: Grid,
    instr: Instructions,
}

impl Simulation {
    fn new(grid: Grid, instr: Instructions) -> Self {
        Simulation { grid, instr }
    }

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

fn read_input() -> io::Result<(Grid, Instructions)> {
    let path = "day15.txt"; // File path
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let re = Regex::new(
        r"(?x)
         (?P<m1>\#+.+\#) |
         (?P<m2>(^|>|<|v).+)",
    )
    .unwrap();

    let mut grid: Grid = Vec::new();
    let mut instructions: Instructions = Vec::new();

    for line in (&mut reader).lines() {
        let line_str = line.unwrap();

        for mat in re.captures_iter(&line_str) {
            if mat.name("m1").is_some() {
                let mut tmp: Vec<char> = mat.get(0).unwrap().as_str().chars().collect();

                tmp = tmp
                    .iter()
                    .flat_map(|x| match *x {
                        '.' => vec!['.', '.'],
                        'O' => vec!['[', ']'],
                        '#' => vec!['#', '#'],
                        '@' => vec!['@', '.'],
                        _ => vec![],
                    })
                    .collect();

                grid.push(tmp)
            } else if mat.name("m2").is_some() {
                let tmp: Vec<char> = mat.get(0).unwrap().as_str().chars().collect();
                instructions.extend(tmp);
            }
        }
    }

    instructions.reverse();

    Ok((grid, instructions))
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let (grid, instr) = read_input()?;

    utils::print_grid(&grid);

    let mut sim = Simulation::new(grid, instr);
    sim.execute();
    let score = sim.find_score();
    utils::print_grid(&sim.grid);

    let end = start.elapsed();
    println!("Total Score {:?} in {:?}", score, end);

    Ok(())
}
