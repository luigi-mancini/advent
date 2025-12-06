use regex::Regex;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};



fn correct_or_swap(rules : &HashMap<u32, Vec<u32>>, upd: &mut Vec<u32>, i: usize) -> bool
{
    let key = upd[i];

    if let Some(val) = rules.get(&key) {
	for j in (0..i).rev() {
	    //println!("i={} j={}", i, j);

	    if val.contains(&(upd[j] as u32)) {
		upd[i] = upd[j];
		upd[j] = key;
		return false;
	    } else
	    {
		//println!("PASSED {} {}", key, upd[j]);
		continue;
	    }
	}
    }
    true
}
    


fn fix_ordering(rules : &HashMap<u32, Vec<u32>>, upd: &mut Vec<u32>) -> bool
{
    for i in (1..upd.len()).rev() {
	let mut ret = false;
	while !ret {
	    ret = correct_or_swap(rules, upd, i);
	}
    }
    true
}
fn is_correct(rules : &HashMap<u32, Vec<u32>>, upd: &Vec<u32>) -> bool
{
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

    correct
}


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

    //for (k, v) in &rules {
//	println!("Key:{:?} Vec:{:?}", k, v);
//    }


    let mut count = 0;
    
    for upd in &mut update_list {

	let correct = is_correct(&rules, &upd);
	//println!("{}", correct);
	
//	if correct {
//	    println!("{:?} IS CORRECT", upd);

	    
	//	}
	if !correct {
	    //println!("{:?} IS WRONG", upd);

	    fix_ordering(&rules, upd);

	    //println!("{:?}", upd);

	    let correct_now = is_correct(&rules, &upd);
	    println!("{:?} {}", upd, correct_now);
	    

	    let ind = upd.len() / 2;
	    count = count + upd[ind];

	}
	    
    }
    println!("Count {}", count);
    
    Ok(())
}
