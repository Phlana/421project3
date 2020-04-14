// use crate::board::{Board, Cell};
// mod board;
// pub extern crate mongodb;
// pub extern crate r2d2;

mod app_server;

fn main() {
    // let mut b = Board::new();
    // println!("{}", b);

    // b.place(Cell::TYellow, 0);
    // println!("{}", b);

    // b.place(Cell::ORed, 0);
    // println!("{}", b);

    // b.place(Cell::TYellow, 0);
    // println!("{}", b);

    // b.place(Cell::ORed, 0);
    // println!("{}", b);

    // b.place(Cell::TYellow, 1);
    // println!("{}", b);

    // b.place(Cell::ORed, 1);
    // println!("{}", b);
    match app_server::app::initialize() {
        Ok(_) => {},
        Err(e) => {println!("{}", e);}
    }

    app_server::app::run();
}


// run with 
// $env:OPENSSL_DIR = 'C:\vcpkg-master\installed\x64-windows-static'
// $env:OPENSSL_STATIC = 'Yes'
// $env:VCPKGRS_DYNAMIC='1'