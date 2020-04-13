use crate::board_mod::Cell;
use crate::board_mod::board_mod::BoardLike;
use crate::connect4::C4Board;
use crate::toototto::TOBoard;
mod board_mod;
mod connect4;
mod toototto;

fn main() {
    let mut b = C4Board::new(7, 6);
    println!("{}", b);

    // b.place(Cell::TYellow, 0);
    // b.place(Cell::ORed, 1);
    // b.place(Cell::ORed, 2);
    // // b.place(Cell::TYellow, 3);
    // b.place(Cell::TYellow, 4);
    // b.place(Cell::ORed, 5);
    //
    // println!("{}", b);
    // b.place(Cell::TYellow, 3);
    // println!("{}", b);


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
    b.place(Cell::TYellow, 3);
    println!("{}", b);
    // b.place(Cell::TYellow, 1);
    // println!("{}", b);
    // b.place(Cell::TYellow, 2);
    // println!("{}", b);
    b.place(Cell::TYellow, 3);
    println!("{}", b);
}
