#[derive(Clone, PartialEq)]
pub enum Cell {
    TYellow,
    ORed,
    Empty,
}

pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let array = vec![vec![Cell::Empty; height]; width];
        Board {
            width,
            height,
            cells: array,
        }
    }

    fn print_cell(&self, w: usize, h: usize) -> char {
        // let cell = self.cells[w][h];
        match self.cells[w][h] {
            Cell::TYellow => 'T',
            Cell::ORed => 'O',
            Cell::Empty => ' ',
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = String::new();
        for h in 0..self.height {
            // start of board
            string.push('|');
            for w in 0..self.width {
                // string.push(std::char::from_digit(w as u32, 10).unwrap());
                // string.push(std::char::from_digit((self.height - h - 1) as u32, 10).unwrap());
                // string.push(' ');
                string.push(self.print_cell(w, self.height - h - 1));
                string.push('|');
            }
            // new row
            string.push('\n');
        }

        string
    }

    fn find_height(&self, col: usize) -> Result<usize, bool> {
        let mut height = 0usize;
        loop {
            if self.cells[col][height] == Cell::Empty {
                return Ok(height);
            }
            height += 1;
            if height >= self.height {
                return Err(false);
            }
        }
    }

    fn set_cell(&mut self, color: Cell, w: usize, h: usize) {
        self.cells[w][h] = color;
    }

    fn check_cell_color(&self, color: Cell, w: usize, h: usize) -> bool {
        if self.cells[w][h] == color {
            return true;
        }
        false
    }

    pub fn place(&mut self, color: Cell, col: usize) {
        match self.find_height(col) {
            Ok(height) => {
                self.set_cell(color.clone(), col, height);
                if self.check_win(color.clone(), col, height) {
                    println!("game is over");
                }
            },
            Err(_) => { println!("column is full") },
        }
    }

    fn check_win(&self, color: Cell, column: usize, height: usize) -> bool {
        let mut count = 0u8;
        // checking vertical
        // needs to have enough pieces underneath to bother checking
        if height > 2 {
            for h in height-3..height+1 {
                if self.check_cell_color(color.clone(), column, height-h) {
                    count += 1;
                    if count == 4 {
                        return true;
                    }
                }
                else {
                    count = 0;
                    break;
                }
            }
        }

        // checking horizontal
        let left: usize;
        let right: usize;
        if column < 3 {
            left = 0usize;
        }
        else {
            left = column - 3;
        }
        if column > self.width-1-3 {
            right = self.width-1;
        }
        else {
            right = column + 3;
        }
        for c in left..right+1 {
            if self.check_cell_color(color.clone(), c, height) {
                count += 1;
                if count == 4 {
                    return true;
                }
            }
            else {
                count = 0;
            }
        }

        // checking diagonals
        // \diagonal
        count = 0;
        // getting bounds
        let mut top = height as i32;
        let mut left = column as i32;
        let mut bottom = height as i32;
        let mut right= column as i32;
        for i in 0..4 {
            top = height as i32 + 3 - i;
            left = column as i32 - 3 + i;
            if top < self.height as i32 && left >= 0 {
                // valid bound
                break;
            }
        }
        for i in 0..4 {
            bottom = height as i32 - 3 + i;
            right = column as i32 + 3 - i;
            if bottom >= 0 && right < self.width as i32 {
                // valid bound
                break;
            }
        }
        let mut w = left;
        let mut h = top;
        while w < right || h < bottom {
            if self.check_cell_color(color.clone(), w as usize, h as usize) {
                count += 1;
                if count == 4 {
                    return true;
                }
            }
            else {
                count = 0;
            }
            w += 1;
            h -= 1;
        }

        // /diagonal
        count = 0;
        // getting bounds
        top = height as i32;
        left = column as i32;
        bottom = height as i32;
        right= column as i32;
        for i in 0..4 {
            bottom = height as i32 - 3 + i;
            left = column as i32 - 3 + i;
            if bottom >= 0 && left >= 0 {
                // valid bound
                break;
            }
        }
        for i in 0..4 {
            top = height as i32 + 3 - i;
            right = column as i32 + 3 - i;
            if top < self.height as i32 && right < self.width as i32 {
                // valid bound
                break;
            }
        }
        let mut w = left;
        let mut h = bottom;
        while w < right || h < top {
            if self.check_cell_color(color.clone(), w as usize, h as usize) {
                count += 1;
                if count == 4 {
                    return true;
                }
            }
            else {
                count = 0;
            }
            w += 1;
            h += 1;
        }

        false
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
