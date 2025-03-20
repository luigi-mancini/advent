use std::fs::File;
use std::io::{self, BufRead, BufReader};



// S
// .A
// ..M
// ...X

fn CheckDownDiag(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if x == 0 || x == max_x - 1 ||
	y == 0 || y == max_y - 1 {
	    return false;
    }

    (matrix[y-1][x-1] == 'M' && matrix[y+1][x+1] == 'S') ||
	(matrix[y-1][x-1] == 'S' && matrix[y+1][x+1] == 'M')
}

fn CheckUpDiag(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> bool
{
    if x == 0 || x == max_x - 1 ||
	y == 0 || y == max_y - 1 {
	    return false;
    }

    (matrix[y+1][x-1] == 'M' && matrix[y-1][x+1] == 'S') ||
	(matrix[y+1][x-1] == 'S' && matrix[y-1][x+1] == 'M')
}

fn CheckPosition(x: usize, y: usize, max_x: usize, max_y: usize, matrix: &Vec<Vec<char>>) -> u32
{
    println!("{} {} {} {}", x, y, max_x, max_y);
    if matrix[y][x] != 'A' {
	return 0;
    }

    let mut count = 0;
    
    if CheckDownDiag(x, y, max_x, max_y, &matrix) &&
	CheckUpDiag(x, y, max_x, max_y, &matrix) {
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
