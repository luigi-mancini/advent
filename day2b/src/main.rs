use std::any::type_name;
use std::fs::File;
use std::io::{self, BufRead, BufReader};


fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn ascending(s1: &str, s2: &str) -> bool
{
    s1.parse::<i32>().unwrap() <  s2.parse::<i32>().unwrap()
}

fn safe_impl(v: Vec<&str>) -> bool
{
    if v[0] == v[1] {					       
	return false						       
    }							       
								       
    let asc = ascending(v[0], v[1]);				       
    let mut current = v[0].parse::<i32>().unwrap();		       
    for i in v.into_iter().skip(1) {				       
	let int_val = i.parse::<i32>().unwrap();		       
								       
	if asc {						       
	    if int_val <= current || int_val > current + 3 {	       
		return false;					       
	    }							       
	    current = int_val;					       
	} else {						       
	    if int_val >= current || int_val < current - 3 {	       
		return false;					       
	    }							       
	    current = int_val;					       
	}							       
    }							       
    true
}


fn safe(v: &Vec<&str>) -> bool
{
    if v.len() < 2 {
	println!("Report contains less than 2 levels!");
	std::process::exit(0);
    }

    for i in 0..v.len() {
        // Skip the i-th entry

        let filtered_vec = v
            .iter()
            .enumerate()
            .filter(move |&(index, _)| index != i)
            .map(|(_, &value)| value)
	    .collect(); // Extract values

	if safe_impl(filtered_vec) {
	    return true
	}
    }
    
    false
}


fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day2.txt")?;

    // Create a buffered reader
    let mut reader = BufReader::new(file);

    let mut count = 0;
    
    // Read the first line
    for line in (&mut reader).lines() {

    	let line_str = line.unwrap();
    	let nums : Vec<&str> = line_str.split_whitespace().collect();

	let s = safe(&nums);
	println!("{:?} {:?}", nums, s);
	if s {
	    count += 1;
	}
    }
    println!("{:?}", count);

    Ok(())
}
