use regex::{Captures, Regex};

use std::cmp;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Default for Coordinates {
    fn default() -> Self {
        Coordinates { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct ClawMachine {
    button_a: Coordinates,
    button_b: Coordinates,
    prize_location: Coordinates,
}

impl Default for ClawMachine {
    fn default() -> Self {
        ClawMachine {
            button_a: Coordinates::default(),
            button_b: Coordinates::default(),
            prize_location: Coordinates::default(),
        }
    }
}

impl ClawMachine {
    fn get_cost(&mut self) -> usize {
        let max_a = cmp::min(
            (self.prize_location.x / self.button_a.x) + 1,
            (self.prize_location.y / self.button_a.y) + 1,
        );

        let max_b = cmp::min(
            (self.prize_location.x / self.button_b.x) + 1,
            (self.prize_location.y / self.button_b.y) + 1,
        );

        let mut cost = usize::MAX;

        for a in 0..max_a {
            for b in 0..max_b {
                let xpos = a * self.button_a.x + b * self.button_b.x;
                let ypos = a * self.button_a.y + b * self.button_b.y;

                //println!("a {} b {}", a, b);

                if xpos == self.prize_location.x && ypos == self.prize_location.y {
                    cost = cmp::min(cost, a * 3 + b);
                }

                if xpos >= self.prize_location.x || ypos >= self.prize_location.y {
                    break;
                }
            }
        }

        if cost == usize::MAX {
            0
        } else {
            cost
        }
    }
}

fn get_x_y(p1: usize, p2: usize, mat: &Captures<'_>) -> (usize, usize) {
    let x = mat.get(p1).unwrap().as_str();
    let x: usize = x.parse::<usize>().unwrap();

    let y = mat.get(p2).unwrap().as_str();
    let y: usize = y.parse::<usize>().unwrap();

    (x, y)
}

fn read_input() -> io::Result<Vec<ClawMachine>> {
    let path = "test.txt"; // File path
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let re = Regex::new(
        r"(?x)
            (?P<button_a>Button\ A:\ X\+(\d+),\ Y\+(\d+)) |
            (?P<button_b>Button\ B:\ X\+(\d+),\ Y\+(\d+)) |
            (?P<prize>Prize:\ X=(\d+),\ Y=(\d+))
        ",
    )
    .unwrap();

    let mut v: Vec<ClawMachine> = Vec::new();

    let mut cm = ClawMachine::default();

    for line in (&mut reader).lines() {
        let line_str = line.unwrap();

        for mat in re.captures_iter(&line_str) {
            if mat.name("button_a").is_some() {
                let (x, y) = get_x_y(2, 3, &mat);
                cm.button_a = Coordinates { x, y };
            } else if mat.name("button_b").is_some() {
                let (x, y) = get_x_y(5, 6, &mat);
                cm.button_b = Coordinates { x, y };
            } else if mat.name("prize").is_some() {
                let (x, y) = get_x_y(8, 9, &mat);
                cm.prize_location = Coordinates {
                    x: x + 10000000000000,
                    y: y + 10000000000000,
                };
                v.push(cm);
                cm = ClawMachine::default();
            }
        }
    }

    Ok(v)
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let claw_machines = read_input()?;

    let mut cost = 0;
    for mut cm in claw_machines {
        cost += cm.get_cost();
    }
    let end = start.elapsed();
    println!("Total Cost {:?} in {:?}", cost, end);

    Ok(())
}
