use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn create_fs(input: &str, _bytes: usize) -> (Vec<i32>, Vec<usize>) {
    let mut v: Vec<i32> = Vec::new();
    let mut ls: Vec<usize> = Vec::new();

    let mut mode = true;
    let mut counter: i32 = 0;
    for c in input.chars() {
        let ret = c.to_digit(10);
        if let Some(val) = ret {
            if mode {
                v.extend(std::iter::repeat(counter).take(val as usize));
                counter += 1;
            } else {
                ls.extend((v.len()..).take(val as usize));
                v.extend(std::iter::repeat(-1).take(val as usize));
            }
            mode = !mode;
        }
    }
    (v, ls)
}

fn compact(fs: &mut [i32], fl: &mut [usize]) {
    let mut curr_index = fs.len() - 1;
    let mut fl_iter = fl.iter().peekable();

    loop {
        if let Some(free_index) = fl_iter.peek() {
            if curr_index <= **free_index {
                break;
            }

            if fs[curr_index] != -1 {
                fs[**free_index] = fs[curr_index];
                fs[curr_index] = -1;

                fl_iter.next();
            }
            curr_index -= 1;
        } else {
            return;
        }
    }
}

fn calculate_checksum(fs: &[i32]) -> usize {
    let mut cs = 0;
    for (a, b) in fs.iter().enumerate() {
        if *b == -1 {
            return cs;
        }
        cs += a * (*b as usize);
    }
    cs
}

fn main() -> io::Result<()> {
    let file = File::open("day9.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);

    let mut line: String = String::new();
    let num_bytes = reader.read_line(&mut line)?;
    println!("{} {}", num_bytes, line);

    let (mut fs, mut fl) = create_fs(&line, num_bytes);

    //println!("{:?}", fs);
    //println!("{:?}", fl);

    compact(&mut fs, &mut fl);

    //println!("{:?}", fs);
    let cs = calculate_checksum(&fs);

    println!("Checksum {}", cs);

    Ok(())
}
