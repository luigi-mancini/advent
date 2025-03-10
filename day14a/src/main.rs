use regex::{Captures, Regex};

use std::cmp;
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
            pos: { Coordinates { x: pos_x, y: pos_y} },
            vec: { Coordinates { x: vec_x, y: vec_y} }
        }
    }
}

#[derive(Debug, Default)]
struct SecurityGrid {
    robots : Vec<Robot>,
    size_x : isize,
    size_y : isize,
}

impl SecurityGrid {
    fn new(robots: Vec<Robot>, size_x: isize, size_y: isize) -> Self {
        SecurityGrid { robots, size_x, size_y }
    }
}


fn get_num(pos: usize, mat: &Captures<'_>) -> isize {
    let x = mat.get(pos).unwrap().as_str();
    x.parse::<isize>().unwrap()
}

fn read_input() -> io::Result<Vec<Robot>> {
    let path = "test.txt"; // File path
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let re = Regex::new(r"(?x)(?P<entry>p=(\d+),(\d+)\ v=(-?\d+),(-?\d+))").unwrap();

    let mut vec : Vec<Robot> = Vec::new();
    
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
    let grid = SecurityGrid::new(robot_list, 11, 7);

    let mut cost = 0;
//    for mut cm in claw_machines {
//        cost += cm.get_cost();
//    }
    let end = start.elapsed();
    println!("Total Cost {:?} in {:?}", grid, end);

    Ok(())
}
