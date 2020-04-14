#[derive(Clone, PartialEq)]
pub enum Cell {
    TYellow,
    ORed,
    Empty,
}

pub mod board_mod {
    use crate::board_mod::Cell;

    pub trait BoardLikePrivate {
        fn print_cell(&self, w: usize, h: usize) -> char;
        fn find_height(&self, col: usize) -> Result<usize, bool>;
        fn set_cell(&mut self, color: Cell, w: usize, h: usize);
        fn check_win(&self, column: usize, height: usize) -> &str;
    }

    pub trait BoardLike {
        fn new(width: usize, height: usize) -> Self;
        fn place(&mut self, color: Cell, col: usize);
        fn get_width(&self) -> usize;
    }

    pub trait Format {
        fn format(&self) -> String;
    }
}