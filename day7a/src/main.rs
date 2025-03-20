use regex::Regex;

use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn check_results(result: u64, current: &mut u64, operands: &[u64]) -> bool
{
    if *current > result {
	return false;
    }
    //println!("{:?} {:?} {:?}", result, *current, operands);

    match operands {
	[first, rest @ ..] => {

	    let mut temp_current = *current * first;
	    
	    if check_results(result, &mut temp_current, rest) {
		return true;
	    }


	    temp_current = *current + first;
	    check_results(result, &mut temp_current, rest)
	}
	[] => {
	    if *current == result {
		true
	    } else {
		false
	    }
	}
    }
}

fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day7.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);
    let re = Regex::new(r"(?x)
        (\d+)\:\s((\d+)(\s\d+)*) 
        ").unwrap();

   //let mut rules : HashMap<u64, Vec<u64>> = HashMap::new();
   let mut results: Vec<u64> = Vec::new();
   let mut operands: Vec<Vec<u64>>= Vec::new();
    
    for line in (&mut reader).lines() {

    	let line_str = line.unwrap();
	println!("{:?}", line_str);
	
	for mat in re.captures_iter(&line_str) {

	    if mat.get(1).is_some() {

		// Extract the numbers using capturing groups
		let first_number = mat.get(1).unwrap().as_str();
		let first_number: u64 = first_number.parse().unwrap();

		results.push(first_number);
	    }
	    
	    if mat.get(2).is_some() {
		let update = mat.get(2).unwrap().as_str();
		let v = update.split(' ')
		    .filter_map(|num| num.parse::<u64>().ok())
		    .collect::<Vec<u64>>();

		operands.push(v);
	    }
	}

	
    }

    //println!("{:?}", results);
    //println!("{:?}", operands);


    let mut total_calib : u64 = 0;
    
    for (i, res) in results.iter().enumerate() {

	let oper = &mut operands[i];
	let mut first  = oper[0];
	let ret = check_results(*res, &mut first, &oper[1..]);

	if ret {
	    total_calib = total_calib + *res;
	}
	
	println!("{:?} {:?}", oper, ret);
    }

    	println!("{:?}", total_calib);

    
    Ok(())
}
