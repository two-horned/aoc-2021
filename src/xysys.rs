impl Matrix2D {
    pub fn new() -> Matrix2D {
        Matrix2D {list: Vec::new()}
    }

    pub fn get_val(&self, x: isize, y: isize) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        } else if x as usize + 1 > self.x_len() || y as usize + 1 > self.y_len() {
            return None;
        }
        Some(self.list[x as usize][y as usize])
    }

    pub fn get_surround(&self, x: isize, y: isize) -> [Option<u8>;4] {
        [
            self.get_val(x-1, y),
            self.get_val(x, y+1),
            self.get_val(x+1, y),
            self.get_val(x, y-1),
        ]
    }

    pub fn x_len(&self) -> usize {
        self.list.len()
    }

    pub fn y_len(&self) -> usize {
        if self.x_len() == 0 {
            return 0;
        }
        self.list[0].len()
    }

    pub fn push_line(&mut self, vec: Vec<u8>) -> bool {
        if self.x_len() == 0 || self.y_len() == vec.len() {
            self.list.push(vec);
            return true;
        }
        false
    }

    pub fn get_list(&self) -> &Vec<Vec<u8>> {
        &self.list
    }
}

#[derive(Debug)]
pub struct Matrix2D {
    list: Vec<Vec<u8>>,
}
