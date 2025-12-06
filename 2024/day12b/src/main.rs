use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::Instant;

type Node = (char, bool);

struct PlotAnalyzer {
    plot: Vec<Vec<Node>>,
    top: HashMap<usize, Vec<usize>>,
    bottom: HashMap<usize, Vec<usize>>,
    left: HashMap<usize, Vec<usize>>,
    right: HashMap<usize, Vec<usize>>,
}

impl PlotAnalyzer {
    fn new(plot: Vec<Vec<Node>>) -> Self {
        PlotAnalyzer {
            plot,
            top: HashMap::new(),
            bottom: HashMap::new(),
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }

    fn check_up(&mut self, val: char, y: usize, x: usize, nodes: &mut Vec<(usize, usize)>) {
        if y == 0 || self.plot[y - 1][x].0 != val {
            self.top.entry(y).or_default().push(x);
            return;
        }

        if !self.plot[y - 1][x].1 {
            nodes.push((y - 1, x));
        }
    }

    fn check_down(&mut self, val: char, y: usize, x: usize, nodes: &mut Vec<(usize, usize)>) {
        if y == self.plot.len() - 1 || self.plot[y + 1][x].0 != val {
            self.bottom.entry(y).or_default().push(x);
            return;
        }

        if !self.plot[y + 1][x].1 {
            nodes.push((y + 1, x));
        }
    }

    fn check_left(&mut self, val: char, y: usize, x: usize, nodes: &mut Vec<(usize, usize)>) {
        if x == 0 || self.plot[y][x - 1].0 != val {
            self.left.entry(x).or_default().push(y);
            return;
        }

        if !self.plot[y][x - 1].1 {
            nodes.push((y, x - 1));
        }
    }

    fn check_right(&mut self, val: char, y: usize, x: usize, nodes: &mut Vec<(usize, usize)>) {
        if x == self.plot[0].len() - 1 || self.plot[y][x + 1].0 != val {
            self.right.entry(x).or_default().push(y);
            return;
        }

        if !self.plot[y][x + 1].1 {
            nodes.push((y, x + 1));
        }
    }

    fn calculate_area_and_perim(&mut self, nodes: &mut Vec<(usize, usize)>, area: usize) -> usize {
        if let Some((y, x)) = nodes.pop() {
            if self.plot[y][x].1 {
                return self.calculate_area_and_perim(nodes, area);
            }

            //println!("{} {}", y, x);

            // Mark node as visited
            self.plot[y][x].1 = true;
            let val = self.plot[y][x].0;

            self.check_up(val, y, x, nodes);
            self.check_down(val, y, x, nodes);
            self.check_left(val, y, x, nodes);
            self.check_right(val, y, x, nodes);

            self.calculate_area_and_perim(nodes, area) + 1
        } else {
            area
        }
    }

    fn get_num_walls(map: &mut HashMap<usize, Vec<usize>>) -> usize {
        let mut wall_count: usize = 0;

        for (_key, vec) in map.iter_mut() {
            vec.sort();

            let mut curr_pos = -1;
            for i in vec.iter() {
                if curr_pos == -1 || *i as isize != curr_pos + 1 {
                    //println!("{} {}", key, *i);
                    wall_count += 1;
                }
                curr_pos = *i as isize;
            }
        }
        //        println!("WALLCOUNT {}", wall_count);
        wall_count
    }
    fn get_plot(&mut self, y: usize, x: usize) -> usize {
        if self.plot[y][x].1 {
            return 0;
        }

        let mut nodes = vec![(y, x)];

        self.top.clear();
        self.bottom.clear();
        self.left.clear();
        self.right.clear();
        let area = self.calculate_area_and_perim(&mut nodes, 0);
        let walls = Self::get_num_walls(&mut self.top)
            + Self::get_num_walls(&mut self.bottom)
            + Self::get_num_walls(&mut self.left)
            + Self::get_num_walls(&mut self.right);

        //      println!("Area {}  Perim {}", area, walls);

        area * walls
    }

    fn calculate_cost(&mut self) -> usize {
        let mut total: usize = 0;
        for y in 0..self.plot.len() {
            for x in 0..self.plot[y].len() {
                total += self.get_plot(y, x);
            }
        }
        total
    }
}

fn read_input() -> io::Result<Vec<Vec<Node>>> {
    let path = "day12.txt"; // File path
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let matrix: Vec<Vec<Node>> = reader
        .lines()
        .map(|l| l.unwrap().chars().map(|c| (c, false)).collect())
        .collect();

    //println!("{:?}", matrix);
    Ok(matrix)
}

fn main() -> io::Result<()> {
    let mut analyzer = PlotAnalyzer::new(read_input()?);
    let start = Instant::now();
    let total_cost = analyzer.calculate_cost();
    let end = start.elapsed();

    println!("Total Cost {:?} in {:?}", total_cost, end);

    Ok(())
}
