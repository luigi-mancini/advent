use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day3.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut count = 0;

    for line in (&mut reader).lines() {

    	let line_str = line.unwrap();
	
	for mat in re.captures_iter(&line_str) {
            // Extract the numbers using capturing groups
            let first_number = mat.get(1).unwrap().as_str();
            let second_number = mat.get(2).unwrap().as_str();
	    
            // Convert the numbers to integers
            let first_number: u32 = first_number.parse().unwrap();
            let second_number: u32 = second_number.parse().unwrap();

	    count = count + (first_number * second_number);
            println!("Parsed numbers: {}, {} {}", first_number, second_number, count);
	}

	
    }
    
    Ok(())
}
