use std::fs::File;
use std::io::{self, BufRead, BufReader};

type DiskLayout = Vec<i32>;
type FreeList = Vec<(usize, usize)>;
type FileList = Vec<(usize, usize)>;

type FileSystem = (DiskLayout, FreeList, FileList);

fn create_fs(input: &str, _bytes: usize) -> FileSystem {
    let mut v: Vec<i32> = Vec::new();
    let mut free_list: Vec<(usize, usize)> = Vec::new();
    let mut file_list: Vec<(usize, usize)> = Vec::new();

    let mut mode = true;
    let mut counter: i32 = 0;
    for c in input.chars() {
        let ret = c.to_digit(10);
        if let Some(val) = ret {
            if mode {
                file_list.push((v.len(), val as usize));
                v.extend(std::iter::repeat(counter).take(val as usize));
                counter += 1;
            } else {
                free_list.push((v.len(), val as usize));
                v.extend(std::iter::repeat(-1).take(val as usize));
            }
            mode = !mode;
        }
    }
    (v, free_list, file_list)
}

fn find_free_space(index: usize, len: usize, free_ls: &mut [(usize, usize)]) -> Option<usize> {
    if let Some(entry) = free_ls
        .iter_mut()
        .find(|(fl_indx, fl_len)| len <= *fl_len && index > *fl_indx)
    {
        let ret_val = entry.0;
        entry.0 += len;
        entry.1 -= len;
        Some(ret_val)
    } else {
        None
    }
}

fn compact(fs: &mut [i32], free_ls: &mut [(usize, usize)], file_ls: &[(usize, usize)]) {
    for (indx, len) in file_ls.iter().rev() {
        if let Some(val) = find_free_space(*indx, *len, free_ls) {
            let id = fs[*indx];

            for entry in fs.iter_mut().skip(val).take(*len) {
                *entry = id;
            }

            for entry in fs.iter_mut().skip(*indx).take(*len) {
                *entry = -1;
            }
        }
    }
}

fn calculate_checksum(fs: &[i32]) -> usize {
    let mut cs = 0;
    for (a, b) in fs.iter().enumerate() {
        if *b != -1 {
            cs += a * (*b as usize);
        }
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

    let (mut fs, mut free_list, file_list) = create_fs(&line, num_bytes);

    //println!("{:?}", fs);
    //println!("{:?}", fl);

    compact(&mut fs, &mut free_list, &file_list);

    println!("{:?}", fs);
    let cs = calculate_checksum(&fs);

    println!("Checksum {}", cs);

    Ok(())
}
