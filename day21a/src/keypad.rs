use crate::coordinates::Coordinates;
use std::collections::HashMap;

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
        let new_x = current.x as i32 + x_off;

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
                    if y == 3 && x == 2 {
                        println!("finding paths for {} {} to  {} {}", y, x, end.y, end.x);
                    }

                    let current = Coordinates::new(y, x);
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

    pub fn decode(&self, start: char, code: &String) -> Vec<String> {
        let mut ret = Vec::new();

        if let Some(end) = code.chars().next() {
            if let Some(path) = self.paths.get(&(start, end)) {
                for p in path {
                    let mut tmp = p.clone();
                    tmp.push('A');
                    ret.push(tmp);
                }
            } else {
                return ret;
            }
        } else {
            return ret;
        }

        let chars: Vec<char> = code.chars().collect();

        for pair in chars.windows(2) {
            if let Some(paths) = self.paths.get(&(pair[0], pair[1])) {
                let mut tmp_vec = Vec::new();
                for p1 in ret.into_iter() {
                    for p2 in paths {
                        let mut tmp_str = p1.clone();
                        tmp_str.push_str(p2);
                        tmp_str.push('A');
                        tmp_vec.push(tmp_str);
                    }
                }
                ret = tmp_vec;
            }
        }

        ret
    }

    pub fn decode_vec(&self, start: char, codes: &Vec<String>) -> Vec<String> {
        let mut ret = Vec::new();

        for c in codes {
            let tmp = self.decode(start, c);
            ret.extend(tmp);
        }

        ret
    }

    pub fn decode_len(&self, start: char, vec: &Vec<String>, iter_count: usize) -> usize {
        let mut input = vec.clone();
        let mut out: Vec<String> = Vec::new();

        for _ in 0..iter_count {
            for i in input {
                let tmp = self.decode(start, &i);
                if !tmp.is_empty() {
                    if out.is_empty() || tmp[0].len() < out[0].len() {
                        out = tmp;
                    }
                }
            }

            input = out;
            out = vec![];
        }

        if !input.is_empty() {
            input[0].len()
        } else {
            0
        }
    }
}
