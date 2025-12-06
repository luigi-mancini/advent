use std::fs::File;
use std::io::{self, BufRead, BufReader};



// S
// .A
// ..M
// ...X

fn CheckUp(x: usize, y: usize, _max_x: usize, _max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if y < 3 {
	return false;
    }
    matrix[y-1][x] == 'M' && matrix[y-2][x] == 'A' && matrix[y-3][x] == 'S' 
}

fn CheckDown(x: usize, y: usize, _max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if y > (max_y - 4) {
	return false;
    }
    matrix[y+1][x] == 'M' && matrix[y+2][x] == 'A' && matrix[y+3][x] == 'S' 
}

fn CheckLeft(x: usize, y: usize, _max_x: usize, _max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if x < 3 {
	return false;
    }
    matrix[y][x-1] == 'M' && matrix[y][x-2] == 'A' && matrix[y][x-3] == 'S' 
}

fn CheckRight(x: usize, y: usize, max_x: usize, _max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if x > (max_x - 4) {
	return false;
    }
    matrix[y][x+1] == 'M' && matrix[y][x+2] == 'A' && matrix[y][x+3] == 'S' 
}
////////////////////////////////////////////
fn CheckUpLeft(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if y < 3 || x < 3 {
	return false;
    }
    matrix[y-1][x-1] == 'M' && matrix[y-2][x-2] == 'A' && matrix[y-3][x-3] == 'S' 
}

fn CheckDownLeft(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if y > (max_y - 4) || x < 3 {
	return false;
    }
    matrix[y+1][x-1] == 'M' && matrix[y+2][x-2] == 'A' && matrix[y+3][x-3] == 'S' 
}

fn CheckUpRight(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if y < 3 || x > (max_x - 4) {
	return false;
    }
    matrix[y-1][x+1] == 'M' && matrix[y-2][x+2] == 'A' && matrix[y-3][x+3] == 'S' 
}

fn CheckDownRight(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if y > (max_y - 4) || x > (max_x - 4) {
	return false;
    }
    matrix[y+1][x+1] == 'M' && matrix[y+2][x+2] == 'A' && matrix[y+3][x+3] == 'S' 
}

fn CheckPosition(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> u32
{
    println!("{} {} {} {}", x, y, max_x, max_y);
    if matrix[y][x] != 'X' {
	return 0;
    }

    let mut count = 0;
    
    if CheckUp(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
    if CheckDown(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
    if CheckRight(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
    if CheckLeft(x, y, max_x, max_y, &matrix) {
	count += 1;
    }

    if CheckUpLeft(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
    if CheckUpRight(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
    if CheckDownLeft(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
    if CheckDownRight(x, y, max_x, max_y, &matrix) {
	count += 1;
    }
	
    count
}

fn main() -> io::Result<()>
{
    // Open the file
    let file = File::open("day4.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {

    	let line = line?;

	let row: Vec<char> = line
	    .chars()
	    .collect();

	println!("{:?}", row);
	
	matrix.push(row);

    }


    let max_x = matrix[0].len();
    let max_y = matrix.len();

    let mut count = 0;
    
    for x in 0..max_x {
	for y in 0..max_y {
	    count += CheckPosition(x, y, max_x, max_y, &matrix);
	}
    }
    println!("{} {} {}", max_x, max_y, count);
   
    Ok(())
}
