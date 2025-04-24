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
        if design == buf {
            //println!("Matched {}", design);
            *count += 1;
            return true;
        }

        let start = buf.len();

        for t in self.towels.iter() {
            let end = start + t.len();
            if design.len() < end || design[start..end] != *t {
                continue;
            }

            buf.push_str(t);
            match self.cache.borrow().get(&(design.len() - buf.len())) {
                Some(val) => {
                    *count += *val;
                    buf.truncate(buf.len() - t.len());
                    return true;
                }
                None => {
                    let _ = self.valid_design(design, buf, count);
                    buf.truncate(buf.len() - t.len());
                }
            }
        }
        false
    }

    fn find_valid_designs(&self) -> usize {
        let mut count = 0;
        let mut overall = 0;

        for d in self.designs.iter() {
            self.cache.borrow_mut().clear();

            let mut temp = String::new();
            let mut total_count = 0;
            if self.valid_design(d, &mut temp, &mut total_count) {
                count += 1;
            }
            overall += total_count;
            println!("MY TOTAL COUNT IS {}", total_count);
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
