#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Coordinates {
    pub y: usize,
    pub x: usize,
}

impl Coordinates {
    pub fn new(y: usize, x: usize) -> Self {
        Coordinates { y, x }
    }

    pub fn distance(c1: Coordinates, c2: Coordinates) -> usize {
        let x = if c1.x > c2.x {
            c1.x - c2.x
        } else {
            c2.x - c1.x
        };
        let y = if c1.y > c2.y {
            c1.y - c2.y
        } else {
            c2.y - c1.y
        };

        x + y
    }
}
