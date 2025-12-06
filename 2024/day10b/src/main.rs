use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn score_up(
    current_val: u32,
    current_score: u32,
    start: (usize, usize),
    matrix: &[Vec<u32>],
    nodes: &mut HashSet<(usize, usize)>,
) -> u32 {
    //println!("SCORE UP {:?}", start);
    if start.0 == 0 || matrix[start.0 - 1][start.1] != (current_val + 1) {
        //println!("DEAD END");
        return current_score;
    }

    if current_val + 1 == 9 {
        //println!("Found a 9");
        nodes.insert((start.0 - 1, start.1));
        return current_score + 1;
    }

    score_up(
        current_val + 1,
        current_score,
        (start.0 - 1, start.1),
        matrix,
        nodes,
    ) + score_left(
        current_val + 1,
        current_score,
        (start.0 - 1, start.1),
        matrix,
        nodes,
    ) + score_right(
        current_val + 1,
        current_score,
        (start.0 - 1, start.1),
        matrix,
        nodes,
    )
}

fn score_down(
    current_val: u32,
    current_score: u32,
    start: (usize, usize),
    matrix: &[Vec<u32>],
    nodes: &mut HashSet<(usize, usize)>,
) -> u32 {
    //println!("SCORE DOWN {:?}", start);
    if start.0 == matrix.len() - 1 || matrix[start.0 + 1][start.1] != (current_val + 1) {
        //println!("DEAD END");
        return current_score;
    }

    if current_val + 1 == 9 {
        //println!("Found a 9");
        nodes.insert((start.0 + 1, start.1));
        return current_score + 1;
    }

    score_down(
        current_val + 1,
        current_score,
        (start.0 + 1, start.1),
        matrix,
        nodes,
    ) + score_left(
        current_val + 1,
        current_score,
        (start.0 + 1, start.1),
        matrix,
        nodes,
    ) + score_right(
        current_val + 1,
        current_score,
        (start.0 + 1, start.1),
        matrix,
        nodes,
    )
}

fn score_left(
    current_val: u32,
    current_score: u32,
    start: (usize, usize),
    matrix: &[Vec<u32>],
    nodes: &mut HashSet<(usize, usize)>,
) -> u32 {
    //println!("SCORE LEFT {:?}", start);
    if start.1 == 0 || matrix[start.0][start.1 - 1] != (current_val + 1) {
        //println!("DEAD END");
        return current_score;
    }

    if current_val + 1 == 9 {
        //println!("Found a 9");
        nodes.insert((start.0, start.1 - 1));
        return current_score + 1;
    }

    score_left(
        current_val + 1,
        current_score,
        (start.0, start.1 - 1),
        matrix,
        nodes,
    ) + score_up(
        current_val + 1,
        current_score,
        (start.0, start.1 - 1),
        matrix,
        nodes,
    ) + score_down(
        current_val + 1,
        current_score,
        (start.0, start.1 - 1),
        matrix,
        nodes,
    )
}

fn score_right(
    current_val: u32,
    current_score: u32,
    start: (usize, usize),
    matrix: &[Vec<u32>],
    nodes: &mut HashSet<(usize, usize)>,
) -> u32 {
    //println!("SCORE RIGHT {:?}", start);
    if start.1 == matrix[0].len() - 1 || matrix[start.0][start.1 + 1] != (current_val + 1) {
        //println!("DEAD END");
        return current_score;
    }

    if current_val + 1 == 9 {
        //println!("Found a 9");
        nodes.insert((start.0, start.1 + 1));
        return current_score + 1;
    }

    score_right(
        current_val + 1,
        current_score,
        (start.0, start.1 + 1),
        matrix,
        nodes,
    ) + score_up(
        current_val + 1,
        current_score,
        (start.0, start.1 + 1),
        matrix,
        nodes,
    ) + score_down(
        current_val + 1,
        current_score,
        (start.0, start.1 + 1),
        matrix,
        nodes,
    )
}

fn score_trailhead(start: (usize, usize), matrix: &[Vec<u32>]) -> u32 {
    //println!("scoring trailhead {:?}", start);

    let mut nodes: HashSet<(usize, usize)> = HashSet::new();
    let s1 = score_up(0, 0, start, matrix, &mut nodes);
    let s2 = score_down(0, 0, start, matrix, &mut nodes);
    let s3 = score_left(0, 0, start, matrix, &mut nodes);
    let s4 = score_right(0, 0, start, matrix, &mut nodes);

    let score = s1 + s2 + s3 + s4;
    //println!("scoring trailhead {} {} {} {} {}", score, s1, s2, s3, s4);
    nodes.len() as u32
    //score
}

fn main() -> io::Result<()> {
    // Open the file
    let file = File::open("day10.txt")?;

    // Create a buffered reader
    let reader = BufReader::new(file);

    let mut matrix: Vec<Vec<u32>> = Vec::new();
    let mut trail_heads: Vec<(usize, usize)> = Vec::new();

    for line in reader.lines() {
        let line = line?;

        let row: Vec<u32> = line.chars().filter_map(|num| num.to_digit(10)).collect();

        for (index, _) in row.iter().enumerate().filter(|(_, x)| **x == 0) {
            trail_heads.push((matrix.len(), index));
        }

        //println!("{:?}", row);

        matrix.push(row);
    }

    println!("{:?}", matrix);
    println!("{:?}", trail_heads);

    let score: u32 = trail_heads
        .iter()
        .map(|entry| score_trailhead(*entry, &matrix))
        .sum();

    println!("SCORE {:?}", score);

    Ok(())
}
