use regex::{Captures, Regex};

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug, Default)]
struct Coordinates {
    x: usize,
    y: usize,
}

#[derive(Debug, Default)]
struct ClawMachine {
    button_a: Coordinates,
    button_b: Coordinates,
    prize_location: Coordinates,
}

impl ClawMachine {
    fn cramers_get_cost(&mut self) -> usize {
        let det = self.button_a.x as isize * self.button_b.y as isize
            - self.button_a.y as isize * self.button_b.x as isize;
        let det_a = self.prize_location.x as isize * self.button_b.y as isize
            - self.prize_location.y as isize * self.button_b.x as isize;
        let det_b = self.button_a.x as isize * self.prize_location.y as isize
            - self.button_a.y as isize * self.prize_location.x as isize;

        let a = det_a / det;
        let b = det_b / det;

        if a < 0 || b < 0 {
            return 0;
        }

        let a = a as usize;
        let b = b as usize;

        if self.button_a.x * a + self.button_b.x * b == self.prize_location.x {
            if self.button_a.y * a + self.button_b.y * b == self.prize_location.y {
                return a * 3 + b;
            }
        }
        0
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
    let path = "day13.txt"; // File path
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
        //println!("ClawMachine {:?}", cm);
        cost += cm.cramers_get_cost();
    }
    let end = start.elapsed();
    println!("Total Cost {:?} in {:?}", cost, end);

    Ok(())
}
