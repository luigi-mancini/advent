use anyhow::Result;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

#[derive(Debug)]
struct Onsen {
    towels: Vec<String>,
    designs: Vec<String>,

    cache: RefCell<HashMap<usize, usize>>,
}

impl Onsen {
    fn new(towels: Vec<String>, designs: Vec<String>) -> Self {
        Onsen {
            towels,
            designs,
            cache: RefCell::new(HashMap::new()),
        }
    }

    fn valid_design(&self, design: &str, buf: &mut String, count: &mut usize) -> bool {
        // valid_design is recursively called for each position in the string. local_count
        // is the number of ways a design[n..] can be made
        let mut local_count = 0;

        if design == buf {
            *count += 1;
            return true;
        }

        let start = buf.len();
        let mut return_val = false;

        for t in self.towels.iter() {
            let end = start + t.len();
            if design.len() < end || design[start..end] != *t {
                continue;
            }

            buf.push_str(t);
            let x = self
                .cache
                .borrow()
                .get(&(design.len() - buf.len()))
                .copied();
            match x {
                Some(val) => {
                    *count += val;
                    local_count += val;
                    return_val = true;
                }
                None => {
                    let mut temp = 0;
                    if self.valid_design(design, buf, &mut temp) {
                        return_val = true;
                    }
                    *count += temp;
                    local_count += temp;
                }
            }
            buf.truncate(buf.len() - t.len());
        }

        self.cache
            .borrow_mut()
            .insert(design.len() - buf.len(), local_count);
        return_val
    }

    fn find_valid_designs(&self) -> usize {
        let mut overall = 0;

        for d in self.designs.iter() {
            println!("DESIGN {}", d);

            self.cache.borrow_mut().clear();

            let mut temp = String::new();
            let mut total_count = 0;

            self.valid_design(d, &mut temp, &mut total_count);
            overall += total_count;
        }

        overall
    }
}

fn read_input(path: &str) -> Result<Onsen> {
    let file = File::open(path)?;
    let mut reader = io::BufReader::new(file);

    let mut buf = String::new();
    reader.read_line(&mut buf)?;

    let towels: Vec<_> = buf.split(',').map(|x| x.trim().to_string()).collect();

    reader.read_line(&mut buf)?;

    let designs: Vec<String> = reader
        .lines()
        .filter(|line| match line {
            Ok(l) => !l.trim().is_empty(),
            Err(_) => true,
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Onsen::new(towels, designs))
}

fn main() -> Result<()> {
    let start = Instant::now();
    let onsen = read_input("day19.txt")?;
    println!("{:?}", onsen);

    let count = onsen.find_valid_designs();

    let end = start.elapsed();
    println!("Total Count {:?} in {:?}", count, end);

    Ok(())
}
