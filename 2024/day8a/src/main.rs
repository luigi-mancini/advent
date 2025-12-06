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

            let ix1 = *x1 as isize;
            let ix2 = *x2 as isize;

            let iy1 = *y1 as isize;
            let iy2 = *y2 as isize;

            //println!("{} {} {} {}", x1, y1, x2, y2);
            let anti_x = ix1 + (ix1 - ix2);
            let anti_y = iy1 + (iy1 - iy2);
            if anti_y >= 0 && anti_y < len_y && anti_x >= 0 && anti_x < len_x {
                //println!("{} {}", anti_y, anti_x);
                nodes.insert((anti_y, anti_x));
            }
        }
        //println!();
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
