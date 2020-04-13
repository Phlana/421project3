use crate::board_mod::Cell;
use crate::board_mod::board_mod::*;

pub struct C4Board {
    width: usize,
    height: usize,
    cols_filled: usize,
    cells: Vec<Vec<Cell>>,
}

impl BoardLike for C4Board {
    fn new(width: usize, height: usize) -> Self {
        let array = vec![vec![Cell::Empty; height]; width];
        C4Board {
            width,
            height,
            cols_filled: 0,
            cells: array,
        }
    }

    fn place(&mut self, color: Cell, col: usize) {
        match self.find_height(col) {
            Ok(height) => {
                self.set_cell(color.clone(), col, height);
                if height == self.height - 1 {
                    // last spot in column filled
                    println!("column filled");
                    self.cols_filled += 1;
                    if self.cols_filled == self.width {
                        // all columns filled
                        println!("board is full");
                    }
                }
                match self.check_win( col, height) {
                    "red" => { println!("red wins") },
                    "yellow" => { println!("yellow wins") },
                    _ => {},
                }
            },
            Err(_) => { println!("column is full") },
        }
    }
}

impl BoardLikePrivate for C4Board {
    fn print_cell(&self, w: usize, h: usize) -> char {
        match self.cells[w][h] {
            Cell::TYellow => 'Y',
            Cell::ORed => 'R',
            Cell::Empty => '_',
        }
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

    fn check_win(&self, column: usize, height: usize) -> &str {
        let mut rcount = 0u8;
        let mut ycount = 0u8;
        let red = Cell::ORed;
        let yellow = Cell::TYellow;
        // checking vertical
        // needs to have enough pieces underneath to bother checking
        if height > 2 {
            for h in height-3..height+1 {
                if self.check_cell_color(red.clone(), column, height-h) {
                    ycount = 0;
                    rcount += 1;
                    if rcount == 4 {
                        return "red";
                    }
                }
                else {
                    rcount = 0;
                    ycount += 1;
                    if ycount == 4 {
                        return "yellow";
                    }
                }
            }
            // winner not fount, reset counts
            rcount = 0;
            ycount = 0;
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
            if self.check_cell_color(red.clone(), c, height) {
                ycount = 0;
                rcount += 1;
                if rcount == 4 {
                    return "red";
                }
            }
            else if self.check_cell_color(yellow.clone(), c, height) {
                rcount = 0;
                ycount += 1;
                if ycount == 4 {
                    return "yellow";
                }
            }
            else {
                // empty cell, reset both counts
                rcount = 0;
                ycount = 0;
            }
        }
        // winner not fount, reset counts
        rcount = 0;
        ycount = 0;

        // checking diagonals
        // \diagonal
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
        while w < right+1 || h < bottom-1 {
            if self.check_cell_color(red.clone(), w as usize, h as usize) {
                ycount = 0;
                rcount += 1;
                if rcount == 4 {
                    return "red";
                }
            }
            else if self.check_cell_color(yellow.clone(), w as usize, h as usize) {
                rcount = 0;
                ycount += 1;
                if ycount == 4 {
                    return "yellow";
                }
            }
            else {
                // empty cell, reset both counts
                rcount = 0;
                ycount = 0;
            }
            w += 1;
            h -= 1;
        }
        // winner not fount, reset counts
        rcount = 0;
        ycount = 0;

        // /diagonal
        // getting bounds
        let mut top = height as i32;
        let mut left = column as i32;
        let mut bottom = height as i32;
        let mut right= column as i32;
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
        while w < right+1 || h < top+1 {
            if self.check_cell_color(red.clone(), w as usize, h as usize) {
                ycount = 0;
                rcount += 1;
                if rcount == 4 {
                    return "red";
                }
            }
            else if self.check_cell_color(yellow.clone(), w as usize, h as usize) {
                rcount = 0;
                ycount += 1;
                if ycount == 4 {
                    return "yellow";
                }
            }
            else {
                // empty cell, reset both counts
                rcount = 0;
                ycount = 0;
            }
            w += 1;
            h += 1;
        }

        // no win detected
        return "";
    }
}

impl C4Board {




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





    fn check_cell_color(&self, color: Cell, w: usize, h: usize) -> bool {
        if self.cells[w][h] == color {
            return true;
        }
        false
    }




}

impl std::fmt::Display for C4Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
