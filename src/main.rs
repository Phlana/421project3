use std::io;
use crate::board_mod::Cell;
use crate::board_mod::board_mod::BoardLike;
use crate::connect4::C4Board;
use crate::toototto::TOBoard;
use std::mem::take;

mod board_mod;
mod connect4;
mod toototto;

fn take_turn(player: i32, width: usize) {
    println!("Which column will you place a piece in? (1-{})", width);
    let mut column = String::new();
    io::stdin().read_line(&mut column).expect("failed to read line");
    column = column.trim().to_string();
}

fn start_c4() -> bool {
    let mut active = true;
    let mut board = C4Board::new(7, 6);
    while active {
        print!("{}", board);
        take_turn(0, board.get_width());
    }
    true
}

fn start_toototto() -> bool {
    let mut active = true;
    let mut board = TOBoard::new(6, 4);

    true
}

fn main() {
    let mut finished = false;
    while !finished {
        println!("Which game would you like to play? (connect-4 or toot-otto)");
        let mut game = String::new();
        io::stdin().read_line(&mut game).expect("failed to read line");
        game = game.trim().to_string();

        match game.as_str() {
            "connect-4" => { finished = start_c4(); },
            "toot-otto" => { finished = start_toototto(); },
            "q" | "-q" | "-quit" | "quit" | "exit" => { finished = true; },
            _ => { println!("unrecognised input: {}", game); },
        }
    }
}
