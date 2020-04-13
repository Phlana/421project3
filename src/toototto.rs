#[derive(Clone, PartialEq)]
pub enum Cell {
    TYellow,
    ORed,
    Empty,
}

pub struct Board {
    width: usize,
    height: usize,
    cols_filled: usize,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let array = vec![vec![Cell::Empty; height]; width];
        Board {
            width,
            height,
            cols_filled: 0,
            cells: array,
        }
    }

    fn print_cell(&self, w: usize, h: usize) -> char {
        match self.cells[w][h] {
            Cell::TYellow => 'T',
            Cell::ORed => 'O',
            Cell::Empty => '_',
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
                if height == self.height - 1 {
                    // last spot in column filled
                    println!("column filled");
                    self.cols_filled += 1;
                    if self.cols_filled == self.width {
                        // all columns filled
                        println!("board is full");
                    }
                }
                if self.check_win( col, height) {
                    println!("game is over");
                }
            },
            Err(_) => { println!("column is full") },
        }
    }

    fn check_win(&self, column: usize, height: usize) -> bool {
        let mut toot = false;
        let mut otto = false;
        // checking vertical
        // needs to have enough pieces underneath to bother checking
        if height > 2 {
            let mut s = String::new();
            for h in height-3..height+1 {
                s.push(self.print_cell(column, height-h));
            }
            match s.as_str() {
                "TOOT" => { toot = true; },
                "OTTO" => { otto = true; },
                _ => {},
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
        // number of strings for checking
        let num_strings = right + 1 - left - 3;
        let mut strings = vec![String::new(); num_strings];
        let mut counter = 1usize;
        for c in left..right+1 {
            for i in 0..counter {
                if strings[i].len() < 4 {
                    strings[i].push(self.print_cell(c, height));
                }
            }
            if counter < num_strings {
                counter += 1;
            }
        }

        for string in strings {
            match string.as_str() {
                "TOOT" => { toot = true; },
                "OTTO" => { otto = true; },
                _ => {},
            }
        }

        if toot && otto {
            // tie already achieved, no point in checking diagonals
            println!("tie");
            return true;
        }

        // checking diagonals
        // \diagonal
        // getting bounds
        let mut top = height as i32;
        let mut left = column as i32;
        let mut bottom = height as i32;
        let mut right = column as i32;
        // try 4 times to find bound from furthest, stops at its own position
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
        let num_cells = right + 1 - left;
        if num_cells > 3 {
            let mut w = left;
            let mut h = top;
            let num_strings = (num_cells - 3) as usize;
            let mut strings = vec![String::new(); num_strings];
            let mut counter = 1usize;
            while w < right+1 || h < bottom-1 {
                for i in 0..counter {
                    if strings[i].len() < 4 {
                        strings[i].push(self.print_cell(w as usize, h as usize));
                    }
                }
                if counter < num_strings {
                    counter += 1;
                }
                w += 1;
                h -= 1;
            }

            for string in strings {
                match string.as_str() {
                    "TOOT" => { toot = true; },
                    "OTTO" => { otto = true; },
                    _ => {},
                }
            }
        }

        if toot && otto {
            // tie already achieved, no point in checking other diagonal
            println!("tie");
            return true;
        }

        // /diagonal
        // getting bounds
        let mut top = height as i32;
        let mut left = column as i32;
        let mut bottom = height as i32;
        let mut right = column as i32;
        // try 4 times to find bound from furthest, stops at its own position
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
        let num_cells = right + 1 - left;
        if num_cells > 3 {
            let mut w = left;
            let mut h = bottom;
            let num_strings = (num_cells - 3) as usize;
            let mut strings = vec![String::new(); num_strings];
            let mut counter = 1usize;
            while w < right+1 || h < top+1 {
                for i in 0..counter {
                    if strings[i].len() < 4 {
                        strings[i].push(self.print_cell(w as usize, h as usize));
                    }
                }
                if counter < num_strings {
                    counter += 1;
                }
                w += 1;
                h += 1;
            }

            for string in strings {
                match string.as_str() {
                    "TOOT" => { toot = true; },
                    "OTTO" => { otto = true; },
                    _ => {},
                }
            }
        }

        if toot && otto {
            // tie
            println!("tie");
            true
        }
        else if toot {
            // toot wins
            println!("toot");
            true
        }
        else if otto {
            // otto wins
            println!("otto");
            true
        }
        else {
            false
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
