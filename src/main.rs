use crate::board::{Board, Cell};

mod board;

fn main() {
    let mut b = Board::new(7, 6);
    println!("{}", b);

    b.place(Cell::TYellow, 0);
    println!("{}", b);

    b.place(Cell::ORed, 0);
    println!("{}", b);

    b.place(Cell::TYellow, 0);
    println!("{}", b);

    b.place(Cell::ORed, 0);
    println!("{}", b);

    b.place(Cell::TYellow, 1);
    println!("{}", b);

    b.place(Cell::TYellow, 1);
    println!("{}", b);
    b.place(Cell::TYellow, 2);
    println!("{}", b);
    b.place(Cell::TYellow, 2);
    println!("{}", b);
    b.place(Cell::TYellow, 2);
    println!("{}", b);
    b.place(Cell::ORed, 3);
    println!("{}", b);
    b.place(Cell::ORed, 3);
    println!("{}", b);
    b.place(Cell::ORed, 3);
    println!("{}", b);
    // b.place(Cell::TYellow, 2);
    // println!("{}", b);
    b.place(Cell::TYellow, 3);
    println!("{}", b);
}
