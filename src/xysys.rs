use std::usize;

impl Matrix2D {

    /** Generates a Matrix2D from a 2D Vector.
       The Length of each individual line must be the same. */
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

    /** Gets the Point at given position or None if the position is not valid */
    pub fn get(&self, x: usize, y: usize) -> Option<&Point> {
        if x >= self.len() || y >= self.wth() {
            return None;
        }
        Some(&self.list[y][x])
    }

    /** Gets a mutable Point at given position or None if the position is not valid */
    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Point> {
        if x >= self.len() || y >= self.wth() {
            return None;
        }
        Some(&mut self.list[y][x])
    }

    /** Sets the Points' value at given position. */
    pub fn set(&mut self, x: usize, y: usize, value: u8) {
        self.list[y][x].value = value;
    }

    fn isize_get(&self, x: isize, y: isize) -> Option<&Point> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        self.get(x, y)
    }
    
    /** Get surrounding points.
     Beginning at bottom, proceeding clockwise. */
    pub fn get_surround(&self, point: &Point) -> [Option<&Point>; 4] {
        let [x, y] = point.get_coord();
        let x = x as isize;
        let y = y as isize;
        [
            // Clockwork
            self.isize_get(x, y-1),
            self.isize_get(x-1, y),
            self.isize_get(x, y+1),
            self.isize_get(x+1, y),
        ]
    }

    /** Get all surrounding Points. 
       Beginning at the bottom, proceeding clockwise, including corners. */
    pub fn get_all_surround(&self, point: &Point) -> [Option<&Point>; 8] {
        let [x, y] = point.get_coord();
        let x = x as isize;
        let y = y as isize;
        [
            // Clockwork
            self.isize_get(x, y-1),
            self.isize_get(x-1,y-1),
            self.isize_get(x-1, y),
            self.isize_get(x-1, y+1),
            self.isize_get(x, y+1),
            self.isize_get(x+1, y+1),
            self.isize_get(x+1, y),
            self.isize_get(x+1,y-1),
        ]
    }

    /** Returns the Length */
    pub fn len(&self) -> usize {
        self.len
    }

    /** Returns the Width */
    pub fn wth(&self) -> usize {
        self.wth
    }
    
    /** A list from which you can cycle through all the points */
    pub fn list(&self) -> Vec<&Point> {
        let mut vec = vec![];
        for points in &self.list {
            for point in points {
                vec.push(point)
            }
        }
        vec
    }

    /** A mutable list from which you can cycle through all the points */
    pub fn list_mut(&mut self) -> Vec<&mut Point> {
        let mut vec = vec![];
        for points in &mut self.list {
            for point in points {
                vec.push(point)
            }
        }
        vec
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

/** A Point contains it's coordinates and a value.
   A Point has fixed coordinates but the value is changable.
   Only makes sense inside a Matrix2D. */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    coordinates: [usize; 2],
    value: u8,
}

/** Basically a XY-Coordinate Field with Points inside */
#[derive(Debug, Clone)]
pub struct Matrix2D {
    list: Vec<Vec<Point>>,
    wth: usize,
    len: usize,
}
