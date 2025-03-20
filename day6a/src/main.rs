use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn step_up(x: &usize, y: &mut usize, matrix: &mut Vec<Vec<char>>) -> bool
{
    if *y == 0 {
	matrix[*y][*x] = 'X';
	return false;
    }

    match matrix[*y-1][*x] {
	'#' => 	{
	    matrix[*y][*x] = '>';
	    true
	}
	_ => {
	    matrix[*y][*x] = 'X';
	    matrix[*y-1][*x] = '^';
	    *y = *y - 1;
	    true
	}
    }
}

fn step_right(x: &mut usize, y: &usize, matrix: &mut Vec<Vec<char>>) -> bool
{
    if *x == matrix[0].len() - 1 {
	matrix[*y][*x] = 'X';
	return false;
    }

    match matrix[*y][*x+1] {
	'#' => 	{
	    matrix[*y][*x] = 'v';
	    true
	}
	_ => {
	    matrix[*y][*x] = 'X';
	    matrix[*y][*x+1] = '>';
	    *x = *x + 1;
	    true
	}
    }
}

fn step_down(x: &usize, y: &mut usize, matrix: &mut Vec<Vec<char>>) -> bool
{
    if *y == matrix.len() - 1 {
	matrix[*y][*x] = 'X';
	return false;
    }

    match matrix[*y+1][*x] {
	'#' => 	{
	    matrix[*y][*x] = '<';
	    true
	}
	_ => {
	    matrix[*y][*x] = 'X';
	    matrix[*y+1][*x] = 'v';
	    *y = *y + 1;
	    true
	}
    }
}

fn step_left(x: &mut usize, y: &usize, matrix: &mut Vec<Vec<char>>) -> bool
{
    if *x == 0 {
	matrix[*y][*x] = 'X';
	return false;
    }

    match matrix[*y][*x-1] {
	'#' => 	{
	    matrix[*y][*x] = '^';
	    true
	}
	_ => {
	    matrix[*y][*x] = 'X';
	    matrix[*y][*x-1] = '<';
	    *x = *x -1;
	    true
	}
    }
}

fn take_step(x: &mut usize, y: &mut usize, matrix: &mut Vec<Vec<char>>) -> bool
{
    match matrix[*y][*x] {
	'^' => step_up(x, y, matrix),
	'>' => step_right(x, y, matrix),
	'v' => step_down(x, y, matrix),
	'<' => step_left(x, y, matrix),
	_   => false
    }
}

fn count_places_visited(matrix: &Vec<Vec<char>>) -> usize
{
    let mut count = 0;
    for m in matrix {
	count = count  +
	    m.iter().filter(|&x| *x == 'X').count();
    }
    count
}
fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day6.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    let mut start_x = 0;
    let mut start_y = 0;
    let mut start_found = false;
    
    for line in reader.lines() {

    	let line = line?;

	let row: Vec<char> = line
	    .chars()
	    .collect();

	if !start_found {
	    if let Some(index) = row.iter().position(|&x| x == '^') {
		start_x = index;
		start_y = matrix.len();
		start_found = true;
	    }
	}
	println!("{:?}", row);
	
	matrix.push(row);

    }

    while take_step(&mut start_x, &mut start_y, &mut matrix) {
	/* for m in &matrix {
	    println!("{:?}", m);
	}
	println!(""); */
    };

    let count = count_places_visited(&matrix);

    println!("Places visitied {}", count);
   
    Ok(())
}
