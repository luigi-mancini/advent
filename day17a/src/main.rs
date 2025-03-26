use regex::{Captures, Regex};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

fn read_input(path: &str) -> Result<usize, regex::Error> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let re = Regex::new(
        r"(?x)
         (?P<m1>Register\ A:\ (\d+)) |
         (?P<m2>Register\ B:\ (\d+)) |
         (?P<m3>Register\ C:\ (\d+)) |
         (?P<m4>Program:\ (.+))",
    )?;

    Ok(0)
}

fn main() -> io::Result<()> {
    let start = Instant::now();

    let grid = read_input("test.txt")?;

    Ok(())
}
