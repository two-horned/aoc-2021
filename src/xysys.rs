impl Matrix2D {

    pub fn from(matrix: Vec<Vec<u8>>) -> Matrix2D {
        let mut list = vec![];
        let wth = matrix[0].len();
        for line in matrix.iter().enumerate() {
            let mut row = vec![];
            if line.1.len() != wth {
                panic!()
            }
            for l in line.1.iter().enumerate() {
                row.push(Point {
                    coordinates: [l.0, line.0],
                    value: *l.1,
                })
            }
            list.push(row)
        }
        Matrix2D { list: list, wth: wth, len: matrix.len() }
    }

    pub fn get_point(&self, x: isize, y: isize) -> Option<&Point> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.len() || y >= self.wth() {
            return None;
        }
        Some(&self.list[y][x])
    }

    /** Get surrounding points.
     Beginning at bottom, proceeding in clock direction. */
    pub fn get_surround(&self, point: &Point) -> [Option<&Point>; 4] {
        let [x, y] = point.get_coord();
        let x = x as isize;
        let y = y as isize;
        [
            // Clockwork
            self.get_point(x, y-1),
            self.get_point(x-1, y),
            self.get_point(x, y+1),
            self.get_point(x+1, y),
        ]
    }

    /** Get even diagonally surrounding points **/
    pub fn get_all_surround(&self, point: Point) -> [Option<&Point>; 8] {
        let [x, y] = point.get_coord();
        let x = x as isize;
        let y = y as isize;
        [
            // Clockwork
            self.get_point(x, y-1),
            self.get_point(x-1,y-1),
            self.get_point(x-1, y),
            self.get_point(x-1, y+1),
            self.get_point(x, y+1),
            self.get_point(x+1, y+1),
            self.get_point(x+1, y),
            self.get_point(x+1,y-1),
        ]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn wth(&self) -> usize {
        self.wth
    }
    
    pub fn list(&self) -> &[Vec<Point>] {
        &self.list
    }
}

impl Point {
    pub fn get(&self) -> u8 {
        self.value
    }
    
    pub fn get_coord(&self) -> [usize; 2] {
        self.coordinates
    }

    pub fn set(&mut self, set: u8) {
        self.value = set;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    coordinates: [usize; 2],
    value: u8,
}

pub struct Matrix2D {
    list: Vec<Vec<Point>>,
    wth: usize,
    len: usize,
}
