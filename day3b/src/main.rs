use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day3.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    let mut count = 0;

    let mut enable_flag = true;
    for line in (&mut reader).lines() {

    	let line_str = line.unwrap();
	
	for mat in re.captures_iter(&line_str) {
	    if mat.get(1).is_some() {
		// Extract the numbers using capturing groups
		let first_number = mat.get(2).unwrap().as_str();
		let second_number = mat.get(3).unwrap().as_str();
	    
		// Convert the numbers to integers
		let first_number: u32 = first_number.parse().unwrap();
		let second_number: u32 = second_number.parse().unwrap();

		if enable_flag {
		    count = count + (first_number * second_number);
		}
		
		println!("Parsed numbers: {}, {} {}", first_number, second_number, count);
	    } else if mat.get(4).is_some() {
		enable_flag = true;
		println!("Found a DO");
	    } else if mat.get(5).is_some() {
		enable_flag = false;
		println!("Found a DONT");
	    }
	}

	
    }
    
    Ok(())
}
