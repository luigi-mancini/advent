use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("day1.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = HashMap::new();

    // Read the first line
    for line in (&mut reader).lines() {

    	let line_str = line.unwrap();
    	let nums : Vec<&str> = line_str.split_whitespace().collect();

	list1.push(nums[0].parse::<i32>().unwrap());
	list2.entry(nums[1].parse::<i32>().unwrap()).and_modify(|i| *i += 1).or_insert(1);
    }

    println!("{:?},{:?}", list1, list2);

    let mut result = 0;
    for &num1 in &list1 {

    	match list2.get(&num1) {
	      Some(val) => result = result + (num1 * val),
	      None => result = result + 0
	}
    }

    println!("Result:{}", result);

    Ok(())
}