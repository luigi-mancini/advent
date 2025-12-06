use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
struct Node {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    val: char,
}

fn step_up(x: &usize, y: &mut usize, matrix: &mut Vec<Vec<Node>>) -> (bool, bool)
{
    if *y == 0 {
        return (false, false);
    }

    if matrix[*y][*x].up {
	return (true, false);
    }
    
    matrix[*y][*x].up = true;
   
    match matrix[*y-1][*x].val {
     	'#' => 	{
     	    matrix[*y][*x].val = '>';
     	}
     	_ => {
     	    matrix[*y][*x].val = '.';
     	    matrix[*y-1][*x].val = '^';
     	    *y = *y - 1;
     	}
    }
    (false, true)
}

fn step_right(x: &mut usize, y: &usize, matrix: &mut Vec<Vec<Node>>) -> (bool, bool)
{
    if *x == matrix[0].len() - 1 {
	return (false, false);
    }

    if matrix[*y][*x].right {
	return (true, false);
    }
	

    match matrix[*y][*x+1].val {
     	'#' => 	{
     	    matrix[*y][*x].val = 'v';
        }
     	_ => {
     	    matrix[*y][*x].val = '.';
     	    matrix[*y][*x+1].val = '>';
     	    *x = *x + 1;
     	}
     }
    (false, true)
}

fn step_down(x: &usize, y: &mut usize, matrix: &mut Vec<Vec<Node>>) -> (bool, bool)
{
    if *y == matrix.len() - 1 {
     	return (false, false);
    }

    if matrix[*y][*x].down {
	return (true, false);
    }
	
    match matrix[*y+1][*x].val {
     	'#' => 	{
     	    matrix[*y][*x].val = '<';
     	}
     	_ => {
     	    matrix[*y][*x].val = '.';
     	    matrix[*y+1][*x].val = 'v';
     	    *y = *y + 1;
     	}
    }
    (false, true)
}

fn step_left(x: &mut usize, y: &usize, matrix: &mut Vec<Vec<Node>>) -> (bool, bool)
{
    if *x == 0 {
     	return (false, false);
    }
    
    if matrix[*y][*x].left {
	return (true, false);
    }

    match matrix[*y][*x-1].val {
     	'#' => 	{
     	    matrix[*y][*x].val = '^';
     	}
     	_ => {
     	    matrix[*y][*x].val = '.';
     	    matrix[*y][*x-1].val = '<';
     	    *x = *x -1;
     	}
     }
    (false, true)
}

fn take_step(x: &mut usize, y: &mut usize, matrix: &mut Vec<Vec<Node>>) -> (bool, bool)
{
    let Node {val, .. } = matrix[*y][*x];
    match val {
	'^' => step_up(x, y, matrix),
	'>' => step_right(x, y, matrix),
	'v' => step_down(x, y, matrix),
	'<' => step_left(x, y, matrix),
	_   => (false, false)
    }
}


fn test_loop(x: usize, y: usize, start_x: usize, start_y: usize, matrix: &Vec<Vec<Node>>) -> bool
{
    let mut m = matrix.clone();
    let mut sx = start_x;
    let mut sy = start_y;

    let Node {val, .. } = matrix[y][x];
    
    if val == '#' || val == '^' {
	return false;
    }

    m[y][x].val = '#';

    loop {
	let (cycle, cont) = take_step(&mut sx, &mut sy, &mut m);
	if cycle {
	    return true;
	}
	if !cont {
	    break;
	}
    }
    false
}


fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day6.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<Node>> = Vec::new();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut start_found = false;
    
    for line in reader.lines() {

    	let line = line?;

	let row: Vec<Node> = line
	    .chars()
	    .map(|val| Node { up: false, down: false, left: false, right:false, val})
	    .collect();

	if !start_found {
	    if let Some(index) = row.iter().position(|x| x.val == '^') {
		start_x = index;
		start_y = matrix.len();
		start_found = true;
	    }
	}
	//println!("{:?}", row);
	
	matrix.push(row);

    }


    let mut count = 0;
    for y in 0 .. matrix.len() {
	for x in 0 .. matrix[y].len() {
	    if test_loop(x, y, start_x, start_y, &matrix) {
		count += 1;
	    }
	}
    }
    
    // while take_step(&mut start_x, &mut start_y, &mut matrix) {
    // 	/* for m in &matrix {
    // 	    println!("{:?}", m);
    // 	}
    // 	println!(""); */
    // };

    // let count = count_places_visited(&matrix);

    println!("Places visitied {}", count);
   
    Ok(())
}
