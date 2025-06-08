use anyhow::{Ok, Result};

fn count_matches(locks: &Vec<[usize; 5]>, keys: &Vec<[usize; 5]>) -> usize {
    let mut count = 0;

    for l in locks {
        for k in keys {

            let tmp = [
                l[0] + k[0],
                l[1] + k[1],
                l[2] + k[2],
                l[3] + k[3],
                l[4] + k[4],
            ];

            if tmp.iter().any(|&x| x > 5) {
                continue; // Skip if any value exceeds 5
            }
            count += 1;
        }
    }
    count
}


fn get_schematic(schematic: &Vec<Vec<char>>) -> [usize; 5] {
    let mut ret: [usize; 5] = [0; 5];

    for row in schematic {
        if row.len() != 5 {
            panic!("Row length is not 5: {:?}", row);
        }

        for (i, &c) in row.iter().enumerate() {
            if c == '#' {
                ret[i] += 1;
            }
        } 
    }

    for i in 0..5 {
        if ret[i] > 0 {
            ret[i] -= 1; // Adjust the count to match the problem's requirements
        }
    }
    ret
}

fn read_input(path: &str) -> Result<(Vec<[usize; 5]>, Vec<[usize; 5]>)> {
    use std::io::BufRead;

    let mut keys : Vec<[usize; 5]> = Vec::new();
    let mut locks : Vec<[usize; 5]> = Vec::new();

    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    let mut tmp_vec: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {

        let line = line?;

        if line.is_empty() {
            let schem = get_schematic(&tmp_vec);

            if tmp_vec[0][0] == '#' {
                println!("Lock: {:?}", schem);

                locks.push(schem);
            } else {
                println!("Key: {:?}", schem);

                keys.push(schem);
            }

            tmp_vec.clear(); // Clear the temporary vector for the next schematic
            continue; 
        }

        let tmp = line.chars().collect::<Vec<char>>();
        tmp_vec.push(tmp);
    }

    let schem = get_schematic(&tmp_vec);

    if tmp_vec[0][0] == '#' {
        println!("Lock: {:?}", schem);

        locks.push(schem);
    } else {
        println!("Key: {:?}", schem);

        keys.push(schem);
    }

    Ok((locks, keys))
}





fn main() -> Result<()> {
    
    let (lock, keys) = read_input("test.txt")?;

    //println!("Locks: {:?}", lock);
    //println!("Keys: {:?}", keys);

    let count = count_matches(&lock, &keys);
    println!("Count: {}", count);

    Ok(())
}
