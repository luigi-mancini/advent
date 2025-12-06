use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("day1.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // Read the first line
    for line in (&mut reader).lines() {

    	let line_str = line.unwrap();
    	let nums : Vec<&str> = line_str.split_whitespace().collect();

	list1.push(nums[0].parse::<i32>().unwrap());
	list2.push(nums[1].parse::<i32>().unwrap());	
    }

    println!("Before sort {:?},{:?}", list1, list2);

    list1.sort();
    list2.sort();

    if list1.len() != list2.len() {
       println!("Size of lists do not match!");
       std::process::exit(0);
    }

    let mut result = 0;
    for (num1, num2) in list1.iter().zip(list2.iter()) {
    	println!("{} {}", num1, num2);

	if num1 > num2 {
	   result = result + (num1 - num2);
	} else {
	   result = result + (num2 - num1);
	}
    }

    println!("Result:{}", result);

    Ok(())
}
