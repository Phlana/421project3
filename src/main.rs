use std::io;
use crate::board_mod::Cell;
use crate::board_mod::board_mod::BoardLike;
use crate::connect4::C4Board;
use crate::toototto::TOBoard;
use std::mem::take;

mod board_mod;
mod connect4;
mod toototto;

fn take_turn(width: usize) -> String {
    println!("Which column will you place a piece in? (1-{})", width);
    let mut column = String::new();
    io::stdin().read_line(&mut column).expect("failed to read line");
    column = column.trim().to_string();

    column
}

fn start_c4() -> bool {
    let mut active = true;
    let mut board = C4Board::new(7, 6);

    // true for yellow turn, false for red turn
    let mut player = true;

    while active {
        print!("{}", board);
        match take_turn(board.get_width()).parse::<usize>() {
            Ok(col) => {
                if player {
                    let result = board.place(Cell::TYellow, col-1);
                    match result {
                        "draw" => {
                            println!("board filled with no winner: draw");
                            break;
                        },
                        "full" => { println!("column is full, try again"); },
                        "red" => {
                            println!("red wins");
                            break;
                        },
                        "yellow" => {
                            println!("yellow wins");
                            break;
                        },
                        _ => { player = false; },
                    }
                }
                else {
                    let result = board.place(Cell::ORed, col-1);
                    match result {
                        "draw" => {
                            println!("board filled with no winner: draw");
                            break;
                        },
                        "full" => { println!("column is full, try again"); },
                        "red" => {
                            println!("red wins");
                            break;
                        },
                        "yellow" => {
                            println!("yellow wins");
                            break;
                        },
                        _ => { player = true; },
                    }
                }
            },
            Err(_) => { println!("not a number, try again") },
        }
    }
    true
}

fn start_toototto() -> bool {
    let mut active = true;
    let mut board = TOBoard::new(6, 4);
    while active {
        print!("{}", board);
        take_turn(board.get_width());
    }
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
