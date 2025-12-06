use regex::{Captures, Regex};

use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug, Default)]
struct Coordinates {
    x: isize,
    y: isize,
}

#[derive(Debug, Default)]
struct Robot {
    pos: Coordinates,
    vec: Coordinates,
}

impl Robot {
    fn new(pos_x: isize, pos_y: isize, vec_x: isize, vec_y: isize) -> Self {
        Robot {
            pos: { Coordinates { x: pos_x, y: pos_y } },
            vec: { Coordinates { x: vec_x, y: vec_y } },
        }
    }
}

#[derive(Debug, Default)]
struct SecurityGrid {
    robots: Vec<Robot>,
    size_x: isize,
    size_y: isize,
}

impl SecurityGrid {
    fn new(robots: Vec<Robot>, size_x: isize, size_y: isize) -> Self {
        SecurityGrid {
            robots,
            size_x,
            size_y,
        }
    }

    fn calculate_final_pos(&mut self) {
        for r in self.robots.iter_mut() {
            for _i in 0..100 {
                r.pos.x += r.vec.x;
                r.pos.y += r.vec.y;

                if r.pos.x >= self.size_x {
                    r.pos.x %= self.size_x;
                } else if r.pos.x < 0 {
                    r.pos.x += self.size_x;
                }

                if r.pos.y >= self.size_y {
                    r.pos.y %= self.size_y;
                } else if r.pos.y < 0 {
                    r.pos.y += self.size_y;
                }

                println!("{} {}", r.pos.x, r.pos.y);
            }
        }
    }

    fn print_grid(&mut self) {
        let mut v = vec![vec![0; self.size_x as usize]; self.size_y as usize];
        for r in self.robots.iter_mut() {
            v[r.pos.y as usize][r.pos.x as usize] += 1;
        }

        for tmp in v.iter() {
            println!("{:?}", tmp);
        }
    }

    fn calc_quads(&mut self) -> usize {
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);

        for r in self.robots.iter() {
            if r.pos.x < self.size_x / 2 && r.pos.y < self.size_y / 2 {
                q1 += 1;
            } else if r.pos.x > self.size_x / 2 && r.pos.y < self.size_y / 2 {
                q2 += 1;
            } else if r.pos.x < self.size_x / 2 && r.pos.y > self.size_y / 2 {
                q3 += 1;
            } else if r.pos.x > self.size_x / 2 && r.pos.y > self.size_y / 2 {
                q4 += 1;
            }
        }

        println!("{} {} {} {}", q1, q2, q3, q4);
        q1 * q2 * q3 * q4
    }
}

fn get_num(pos: usize, mat: &Captures<'_>) -> isize {
    let x = mat.get(pos).unwrap().as_str();
    x.parse::<isize>().unwrap()
}

fn read_input() -> io::Result<Vec<Robot>> {
    let path = "day14.txt"; // File path
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let re = Regex::new(r"(?x)(?P<entry>p=(\d+),(\d+)\ v=(-?\d+),(-?\d+))").unwrap();

    let mut vec: Vec<Robot> = Vec::new();

    for line in (&mut reader).lines() {
        let line_str = line.unwrap();

        for mat in re.captures_iter(&line_str) {
            if mat.name("entry").is_some() {
                let pos_x = get_num(2, &mat);
                let pos_y = get_num(3, &mat);

                let vec_x = get_num(4, &mat);
                let vec_y = get_num(5, &mat);

                vec.push(Robot::new(pos_x, pos_y, vec_x, vec_y));
            }
        }
    }

    Ok(vec)
}

fn main() -> io::Result<()> {
    let start = Instant::now();
    let robot_list = read_input()?;
    let mut grid = SecurityGrid::new(robot_list, 101, 103);

    grid.calculate_final_pos();
    grid.print_grid();

    let cost = grid.calc_quads();

    let end = start.elapsed();
    println!("Total Cost {:?} in {:?}", cost, end);

    Ok(())
}
