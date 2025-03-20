use std::fs::File;
use std::io::{self, BufRead, BufReader};

use std::collections::HashMap;
use std::collections::HashSet;

fn generate_antinodes(
    slice: &[(usize, usize)],
    nodes: &mut HashSet<(isize, isize)>,
    len_x: isize,
    len_y: isize,
) {
    for (y1, x1) in slice {
        for (y2, x2) in slice {
            if y1 == y2 && x1 == x2 {
                continue;
            }

            let x_diff = *x1 as isize - *x2 as isize;
            let y_diff = *y1 as isize - *y2 as isize;

            let mut anti_x = *x1 as isize;
            let mut anti_y = *y1 as isize;

            nodes.insert((anti_y, anti_x));

            loop {
                anti_x += x_diff;
                anti_y += y_diff;

                if anti_y >= 0 && anti_y < len_y && anti_x >= 0 && anti_x < len_x {
                    //println!("{} {}", anti_y, anti_x);
                    nodes.insert((anti_y, anti_x));
                } else {
                    break;
                }
            }
        }
    }
}

fn main() -> io::Result<()> {
    let file = File::open("day8.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut freq: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut nodes: HashSet<(isize, isize)> = HashSet::new();

    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        let row: Vec<char> = line.chars().collect();

        for (a, b) in row.iter().enumerate().filter(|(_, x)| **x != '.') {
            let val = (y, a);
            freq.entry(*b).or_default().push(val);
        }

        matrix.push(row);
    }

    let len_y = matrix.len();
    let len_x = matrix[0].len();

    println!("{:?}", freq);

    for (_, val) in freq {
        generate_antinodes(&val, &mut nodes, len_x as isize, len_y as isize);
    }

    println!("{:?}", nodes);
    println!("{:?}", nodes.len());

    Ok(())
}
