// use std::fmt::{Formatter, Error};

// pub const WIDTH: usize = 7;
// pub const HEIGHT: usize = 6;

// #[derive(Clone, PartialEq)]
// pub enum Cell {
//     TYellow,
//     ORed,
//     Empty,
// }

// pub struct Board {
//     cells: Vec<Vec<Cell>>,
// }

// impl Board {
//     pub fn new() -> Self {
//         let mut array = vec![vec![Cell::Empty; HEIGHT]; WIDTH];
//         Board {
//             cells: array,
//         }
//     }

//     fn print_cell(&self, w: usize, h: usize) -> char {
//         // let cell = self.cells[w][h];
//         match self.cells[w][h] {
//             Cell::TYellow => 'Y',
//             Cell::ORed => 'R',
//             Cell::Empty => ' ',
//         }
//     }

//     pub fn to_string(&self) -> String {
//         let mut string = String::new();
//         for h in 0..HEIGHT {
//             // start of board
//             string.push('|');
//             for w in 0..WIDTH {
//                 string.push(self.print_cell(w, HEIGHT - h - 1));
//                 string.push('|');
//             }
//             // new row
//             string.push('\n');
//             // for row in 0..(WIDTH * 2 - 1) {
//             //     string.push('_');
//             // }
//             // string.push('\n');
//         }

//         string
//     }

//     fn get_height(&self, col: usize) -> Result<usize, bool> {
//         let mut height = 0usize;
//         loop {
//             if self.cells[col][height] == Cell::Empty {
//                 return Ok(height);
//             }
//             height += 1;
//             if height >= HEIGHT {
//                 return Err(false);
//             }
//         }
//     }

//     fn set_cell(&mut self, color: Cell, w: usize, h: usize) {
//         self.cells[w][h] = color;
//     }

//     pub fn place(&mut self, color: Cell, col: usize) {
//         match self.get_height(col) {
//             Ok(height) => { self.set_cell(color, col, height) },
//             Err(False) => { println!("column is full") },
//         }
//     }
// }

// impl std::fmt::Display for Board {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.to_string())
//     }
// }
