mod utils;

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;
use std::{thread, time::Duration};

use regex::Regex;

type Grid = Vec<Vec<char>>;
type Instructions = Vec<char>;

#[derive(Debug, Default)]
struct Simulation {
    grid: Grid,
    instr: Instructions,
}

impl Simulation {
    fn new(grid: Grid, instr: Instructions) -> Self {
        Simulation { grid, instr }
    }

    fn move_up(&mut self, y: &mut usize, x: usize) {
        let mut indx: usize = 1;
        while self.grid[*y - indx][x] == 'O' {
            indx += 1;
        }

        if self.grid[*y - indx][x] == '.' {
            self.grid[*y - indx][x] = self.grid[*y - indx + 1][x];
            self.grid[*y][x] = '.';
            self.grid[*y - 1][x] = '@';
            *y -= 1;
        }
    }

    fn move_down(&mut self, y: &mut usize, x: usize) {
        let mut indx: usize = 1;
        while self.grid[*y + indx][x] == 'O' {
            indx += 1;
        }

        if self.grid[*y + indx][x] == '.' {
            self.grid[*y + indx][x] = self.grid[*y + indx - 1][x];
            self.grid[*y][x] = '.';
            self.grid[*y + 1][x] = '@';
            *y += 1;
        }
    }

    fn move_left(&mut self, y: usize, x: &mut usize) {
        let mut indx: usize = 1;
        while self.grid[y][*x - indx] == 'O' {
            indx += 1;
        }

        if self.grid[y][*x - indx] == '.' {
            self.grid[y][*x - indx] = self.grid[y][*x - indx + 1];
            self.grid[y][*x] = '.';
            self.grid[y][*x - 1] = '@';
            *x -= 1;
        }
    }

    fn move_right(&mut self, y: usize, x: &mut usize) {
        let mut indx: usize = 1;
        while self.grid[y][*x + indx] == 'O' {
            indx += 1;
        }

        if self.grid[y][*x + indx] == '.' {
            self.grid[y][*x + indx] = self.grid[y][*x + indx - 1];
            self.grid[y][*x] = '.';
            self.grid[y][*x + 1] = '@';
            *x += 1;
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
            utils::print_grid(&self.grid);
            thread::sleep(Duration::from_millis(500))
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
                    .map(|(x, val)| if *val == 'O' { y * 100 + x } else { 0 })
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
                let tmp = mat.get(0).unwrap().as_str().chars().collect();
                grid.push(tmp);
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
