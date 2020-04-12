use crate::board::{Board, Cell};

mod board;
mod app_server;

fn main() {
    let mut b = Board::new();
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

    b.place(Cell::ORed, 1);
    println!("{}", b);

    app_server::app::run().unwrap();
}
