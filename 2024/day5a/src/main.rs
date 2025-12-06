use regex::Regex;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day5.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);
    let re = Regex::new(r"(?x)
        ((\d+)\|(\d+))
        |
        (^\d+(,\d+)*)
        ").unwrap();

    let mut rules : HashMap<u32, Vec<u32>> = HashMap::new();
    let mut update_list: Vec<Vec<u32>> = Vec::new();
    
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
		
		//println!("Parsed numbers: {}|{} ", first_number, second_number);
		rules.entry(first_number).or_insert_with(Vec::new).push(second_number);
	    } else if mat.get(4).is_some() {
		let update = mat.get(4).unwrap().as_str();
		let v = update.split(',')
		    .filter_map(|num| num.parse::<u32>().ok())
		    .collect::<Vec<u32>>();
		//println!("{:?}", v);
		update_list.push(v);
	    }
	}

	
    }

    let mut count = 0;
    
    for upd in update_list {

	let mut correct = true;
	
	for i in (1..upd.len()).rev() {
	    let key = upd[i];
	    
	    if let Some(val) = rules.get(&key) {
		for j in (0..i).rev() {
		    //println!("i={} j={}", i, j);

		    if val.contains(&(upd[j] as u32)) {
			correct = false;
			//println!("ERROR {} {}", key, upd[j]);
			break;
		    } else
		    {
			//println!("PASSED {} {}", key, upd[j]);
			continue;
		    }
		}
		if !correct {
		    break;
		}
	    }
	}
	if correct {
	    //println!("{:?} IS CORRECT", upd);

	    let ind = upd.len() / 2;
	    count = count + upd[ind];
	    
	} else {
	    println!("{:?} IS WRONG", upd);
	}
	    
    }
    println!("Count {}", count);
    
    Ok(())
}
