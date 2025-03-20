use std::fs::File;
use std::io::{self, BufRead};

type Node = (char, bool);

struct PlotAnalyzer {
    plot: Vec<Vec<Node>>,
}

impl PlotAnalyzer {
    fn new(plot: Vec<Vec<Node>>) -> Self {
        PlotAnalyzer { plot }
    }

    fn check_up(
        &mut self,
        val: char,
        y: usize,
        x: usize,
        nodes: &mut Vec<(usize, usize)>,
    ) -> usize {
        if y == 0 || self.plot[y - 1][x].0 != val {
            return 1;
        }

        if !self.plot[y - 1][x].1 {
            nodes.push((y - 1, x));
        }

        0
    }

    fn check_down(
        &mut self,
        val: char,
        y: usize,
        x: usize,
        nodes: &mut Vec<(usize, usize)>,
    ) -> usize {
        if y == self.plot.len() - 1 || self.plot[y + 1][x].0 != val {
            return 1;
        }

        if !self.plot[y + 1][x].1 {
            nodes.push((y + 1, x));
        }

        0
    }

    fn check_left(
        &mut self,
        val: char,
        y: usize,
        x: usize,
        nodes: &mut Vec<(usize, usize)>,
    ) -> usize {
        if x == 0 || self.plot[y][x - 1].0 != val {
            return 1;
        }

        if !self.plot[y][x - 1].1 {
            nodes.push((y, x - 1));
        }

        0
    }

    fn check_right(
        &mut self,
        val: char,
        y: usize,
        x: usize,
        nodes: &mut Vec<(usize, usize)>,
    ) -> usize {
        if x == self.plot[0].len() - 1 || self.plot[y][x + 1].0 != val {
            return 1;
        }

        if !self.plot[y][x + 1].1 {
            nodes.push((y, x + 1));
        }

        0
    }

    fn calculate_area_and_perim(
        &mut self,
        nodes: &mut Vec<(usize, usize)>,
        area: usize,
        perim: usize,
    ) -> (usize, usize) {
        if let Some((y, x)) = nodes.pop() {
            if self.plot[y][x].1 {
                return self.calculate_area_and_perim(nodes, area, perim);
            }

            //println!("{} {}", y, x);

            // Mark node as visited
            self.plot[y][x].1 = true;
            let val = self.plot[y][x].0;

            let node_perim = self.check_up(val, y, x, nodes)
                + self.check_down(val, y, x, nodes)
                + self.check_left(val, y, x, nodes)
                + self.check_right(val, y, x, nodes);

            let (temp_area, temp_perim) = self.calculate_area_and_perim(nodes, area, perim);

            (temp_area + 1, temp_perim + node_perim)
        } else {
            (area, perim)
        }
    }

    fn get_plot(&mut self, y: usize, x: usize) -> usize {
        if self.plot[y][x].1 {
            return 0;
        }

        let mut nodes = vec![(y, x)];

        let (area, perim) = self.calculate_area_and_perim(&mut nodes, 0, 0);

        //println!("Area {}  Perim {}", area, perim);

        area * perim
    }
}

fn main() -> io::Result<()> {
    let path = "day12.txt"; // File path
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut matrix: Vec<Vec<Node>> = Vec::new();
    for l in reader.lines() {
        let line = l?;

        let vec: Vec<Node> = line.chars().map(|c| (c, false)).collect();

        //println!("{:?}", vec);
        matrix.push(vec);
    }

    let mut analyzer = PlotAnalyzer::new(matrix);

    let mut total: usize = 0;
    for y in 0..analyzer.plot.len() {
        for x in 0..analyzer.plot[y].len() {
            total += analyzer.get_plot(y, x);
        }
    }

    println!("Total Cost {:?}", total);

    Ok(())
}
