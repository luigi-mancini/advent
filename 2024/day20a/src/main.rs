mod utils;

use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, HashSet};
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

#[derive(Debug)]
struct Maze {
    grid: Grid,
    score_grid: ScoreGrid,
    start: Coordinates,
    end: Coordinates,
    max_y: usize,
    max_x: usize,
    score: usize,
    cheats: [(i32, i32); 8],
    cheat_count: BTreeMap<usize, usize>,
}

impl Maze {
    fn new(grid: Grid, start: Coordinates, end: Coordinates) -> Self {
        let max_y = grid.len();
        let max_x = grid[0].len();

        let cheats = [
            (-2, 0),
            (-1, -1),
            (-1, 1),
            (0, -2),
            (0, 2),
            (1, -1),
            (1, 1),
            (2, 0),
        ];

        Maze {
            grid,
            score_grid: vec![vec![None; max_x]; max_y],
            start,
            end,
            max_y,
            max_x,
            score: 0,
            cheats,
            cheat_count: BTreeMap::new(),
        }
    }

    fn update_score_grid_entry(&mut self, y: usize, x: usize, score: usize) -> bool {
        match self.score_grid[y][x] {
            Some(_val) => false,
            None => {
                self.score_grid[y][x] = Some(score);
                true
            }
        }
    }

    fn create_score_grid(&mut self) -> Result<()> {
        let mut curr: HashSet<Coordinates> = HashSet::new();
        self.score_grid[self.start.y][self.start.x] = Some(0);
        curr.insert(self.start);

        while !curr.is_empty() {
            let mut tmp: HashSet<Coordinates> = HashSet::new();

            for c in curr.iter() {
                let curr_score = (self.score_grid[c.y][c.x])
                    .ok_or_else(|| anyhow!("Unxpcted None value found in ScoreGrid"))?;

                if c.y != 0 && self.grid[c.y - 1][c.x] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y - 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y - 1, c.x));
                    }
                }

                if c.y < self.max_y - 1 && self.grid[c.y + 1][c.x] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y + 1, c.x, score) {
                        tmp.insert(Coordinates::new(c.y + 1, c.x));
                    }
                }

                if c.x != 0 && self.grid[c.y][c.x - 1] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y, c.x - 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x - 1));
                    }
                }

                if c.x < self.max_x - 1 && self.grid[c.y][c.x + 1] != '#' {
                    let score = curr_score + 1;
                    if self.update_score_grid_entry(c.y, c.x + 1, score) {
                        tmp.insert(Coordinates::new(c.y, c.x + 1));
                    }
                }
            }

            curr = tmp;
        }

        self.score = self.score_grid[self.end.y][self.end.x]
            .ok_or_else(|| anyhow!("End location does not have score!"))?;

        Ok(())
    }

    fn check_cheat(
        &mut self,
        cheat: (i32, i32),
        curr: Coordinates,
        min_score: usize,
    ) -> Result<bool> {
        //println!("Checking cheat {:?}!!!", curr);

        if curr == Coordinates::new(91, 104) && cheat == (-2, 0) {
            println!("FOUND THE BAD ONE");
        }

        let x = curr.x as i32 + cheat.1;
        let y = curr.y as i32 + cheat.0;

        if x < 0 || x as usize >= self.max_x || y < 0 || y as usize >= self.max_y {
            return Ok(false);
        }

        let x = x as usize;
        let y = y as usize;

        let start_val =
            self.score_grid[curr.y][curr.x].ok_or_else(|| anyhow!("Cheat start not on path"))?;

        if self.score_grid[y][x].is_some() {
            if Coordinates::new(y, x) == self.end {
                if let Some(end_val) = self.score_grid[y][x] {
                    if end_val >= (start_val + 2 + min_score) {
                        let seconds_saved = end_val - start_val - 2;
                        *(self.cheat_count).entry(seconds_saved).or_insert(0) += 1;

                        println!(
                            "Cheat from {:?} to {:?} saved {}",
                            curr, self.end, seconds_saved
                        );
                        return Ok(true);
                    }
                }
            } else {
                if let Some(end_loc) = self.get_next_location(Coordinates::new(y, x)) {
                    if let Some(end_val) = self.score_grid[end_loc.y][end_loc.x] {
                        if end_val <= self.score {
                            if end_val >= (start_val + 3 + min_score) {
                                let seconds_saved = end_val - start_val - 3;
                                *(self.cheat_count).entry(seconds_saved).or_insert(0) += 1;
                                println!(
                                    "Cheat from {:?} to {:?} saved {}",
                                    curr, end_loc, seconds_saved
                                );
                                return Ok(true);
                            }
                        }
                    }
                }
            }
        }

        Ok(false)
    }

    fn get_next_location(&self, curr: Coordinates) -> Option<Coordinates> {
        if let Some(val) = self.score_grid[curr.y][curr.x] {
            if curr.y != 0 {
                if let Some(next) = self.score_grid[curr.y - 1][curr.x] {
                    if next == val + 1 {
                        return Some(Coordinates::new(curr.y - 1, curr.x));
                    }
                }
            }

            if curr.y != self.max_y - 1 {
                if let Some(next) = self.score_grid[curr.y + 1][curr.x] {
                    if next == val + 1 {
                        return Some(Coordinates::new(curr.y + 1, curr.x));
                    }
                }
            }

            if curr.x != 0 {
                if let Some(next) = self.score_grid[curr.y][curr.x - 1] {
                    if next == val + 1 {
                        return Some(Coordinates::new(curr.y, curr.x - 1));
                    }
                }
            }

            if curr.x != self.max_x - 1 {
                if let Some(next) = self.score_grid[curr.y][curr.x + 1] {
                    if next == val + 1 {
                        return Some(Coordinates::new(curr.y, curr.x + 1));
                    }
                }
            }
        }

        None
    }

    fn find_cheats(&mut self, min_score: usize) -> Result<usize> {
        let mut current_location = self.start;
        let mut cheat_count = 0;

        loop {
            for c in self.cheats {
                if self.check_cheat(c, current_location, min_score)? {
                    cheat_count += 1;
                }
            }

            match self.get_next_location(current_location) {
                Some(val) => current_location = val,
                None => break,
            }
        }

        Ok(cheat_count)
    }
}

fn find_location(grid: &Grid, val: char) -> Option<Coordinates> {
    for (y, row) in grid.iter().enumerate() {
        for (x, entry) in row.iter().enumerate() {
            if *entry == val {
                return Some(Coordinates::new(y, x));
            }
        }
    }
    None
}

fn read_input(path: &str) -> Result<Maze> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let grid: Grid = reader
        .lines()
        .map(|x| match x {
            Ok(val) => Ok(val.chars().collect::<Vec<char>>()),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<_>, _>>()?;

    let start = find_location(&grid, 'S').ok_or_else(|| anyhow!("Start not found in grid"))?;
    let end = find_location(&grid, 'E').ok_or_else(|| anyhow!("End not found in grid"))?;

    Ok(Maze::new(grid, start, end))
}

fn main() -> Result<()> {
    let start = Instant::now();

    let mut maze = read_input("day20.txt")?;

    //utils::print_grid(&maze.grid);

    let _ = maze.create_score_grid();
    //utils::print_grid_opt(&maze.score_grid);

    let cheat_count = maze.find_cheats(100)?;
    let end = start.elapsed();
    println!("The total cheat count is {} {:}?", cheat_count, end);

    /*    for (key, value) in maze.cheat_count {
            if value == 1 {
                println!("There is one cheat that saves {} picoseconds.", key);
            } else {
                println!("There are {} cheats that saves {} picoseconds.", value, key);
            }
        }


    println!("Total Score {:?} in {:?}", score, end);
    */

    Ok(())
}
