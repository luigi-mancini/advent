use crate::coordinates::Coordinates;
use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Keypad {
    keypad: Vec<Vec<Option<char>>>,
    paths: HashMap<(char, char), Vec<String>>,
}

impl Keypad {
    pub fn new(keypad: Vec<Vec<Option<char>>>) -> Self {
        let mut kp = Keypad {
            keypad,
            paths: HashMap::new(),
        };

        kp.init();
        kp
    }

    fn get_locations(&self) -> Vec<Coordinates> {
        let mut v = Vec::new();

        for y in 0..self.keypad.len() {
            for x in 0..self.keypad[0].len() {
                v.push(Coordinates::new(y, x));
            }
        }
        v
    }

    fn find_paths(
        &mut self,
        start: Coordinates,
        current: Coordinates,
        end: Coordinates,
        dist: usize,
        curr_dist: usize,
        path: &mut String,
    ) {
        if curr_dist > dist {
            return;
        }

        if current == end {
            if let Some(start_val) = self.keypad[start.y][start.x] {
                if let Some(end_val) = self.keypad[end.y][end.x] {
                    let tmp = self.paths.entry((start_val, end_val)).or_default();
                    tmp.push(path.clone());
                }
            }
            return;
        }

        if let Some(next) = self.get_next_coord(-1, 0, current) {
            path.push('^');
            self.find_paths(start, next, end, dist, curr_dist + 1, path);
            path.pop();
        }

        if let Some(next) = self.get_next_coord(1, 0, current) {
            path.push('v');
            self.find_paths(start, next, end, dist, curr_dist + 1, path);
            path.pop();
        }

        if let Some(next) = self.get_next_coord(0, -1, current) {
            path.push('<');
            self.find_paths(start, next, end, dist, curr_dist + 1, path);
            path.pop();
        }

        if let Some(next) = self.get_next_coord(0, 1, current) {
            path.push('>');
            self.find_paths(start, next, end, dist, curr_dist + 1, path);
            path.pop();
        }
    }

    fn get_next_coord(
        &mut self,
        y_off: i32,
        x_off: i32,
        current: Coordinates,
    ) -> Option<Coordinates> {
        let new_y = current.y as i32 + y_off;
        let new_x = current.y as i32 + x_off;

        if new_y < 0
            || new_x < 0
            || new_y >= self.keypad.len() as i32
            || new_x >= self.keypad[0].len() as i32
        {
            return None;
        }

        if self.keypad[new_y as usize][new_x as usize].is_some() {
            Some(Coordinates::new(new_y as usize, new_x as usize))
        } else {
            None
        }
    }

    pub fn init(&mut self) {
        let end_locs = self.get_locations();

        for y in 0..self.keypad.len() {
            for x in 0..self.keypad[0].len() {
                for end in end_locs.iter() {
                    let current = Coordinates::new(y, x);
                    if *end != current {
                        let mut str = String::new();
                        self.find_paths(
                            current,
                            current,
                            *end,
                            Coordinates::distance(*end, current),
                            0,
                            &mut str,
                        );
                    }
                }
            }
        }
    }

    pub fn decode(&self, start: char, code: &String) -> Vec<String> {
        let mut ret = Vec::new();

        if let Some(end) = code.chars().next() {
            if let Some(path) = self.paths.get(&(start, end)) {
                ret = path.clone();
            } else {
                return ret;
            }
        } else {
            return ret;
        }

        ret
    }
}
