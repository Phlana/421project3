use std::io;
use crate::board_mod::Cell;
use crate::board_mod::board_mod::BoardLike;
use crate::connect4::C4Board;
use crate::toototto::TOBoard;

mod board_mod;
mod connect4;
mod toototto;

fn get_input() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("failed to read line");
    string = string.trim().to_string();

    string
}

fn take_turn(width: usize) -> usize {
    loop {
        println!("Which column will you place a piece in? (1-{})", width);
        let column = get_input();

        match column.parse::<usize>() {
            Ok(col) => {
                if col > 0 && col <= width {
                    return col;
                }
                else {
                    println!("selection out of bounds, try again");
                }
            },
            Err(_) => {
                println!("not a number, try again");
            },
        }
    }
}

fn start_c4() -> bool {
    let mut board = C4Board::new(7, 6);

    // true for yellow turn, false for red turn
    let mut player = true;

    loop {
        print!("{}", board);
        if player {
            println!("It is yellow's turn");
        }
        else {
            println!("It is red's turn");
        }
        let col = take_turn(board.get_width());
        let result: &str;
        if player {
            result = board.place(Cell::TYellow, col-1);
        }
        else {
            result = board.place(Cell::ORed, col-1);
        }
        match result {
            "draw" => {
                println!("board filled with no winner: draw");
                break;
            },
            "full" => {
                println!("column is full, try again");
            },
            "red" => {
                println!("red wins");
                break;
            },
            "yellow" => {
                println!("yellow wins");
                break;
            },
            _ => {
                // no issues, switch players for next turn
                if player {
                    player = false;
                }
                else {
                    player = true;
                }
            },
        }
    }
    // print the board one last time to show results
    print!("{}", board);

    true
}

// true for T, false for O
fn get_piece() -> bool {
    loop {
        println!("Which piece do you want to place? (T or O)");
        let piece = get_input();

        match piece.as_str() {
            "T" => { return true; },
            "O" => { return false; },
            _ => { println!("not a valid piece, try again"); },
        }
    }
}

fn start_toototto() -> bool {
    let mut board = TOBoard::new(6, 4);

    let mut player = true;

    loop {
        print!("{}", board);
        if player {
            println!("It is toot's turn");
        }
        else {
            println!("It is otto's turn");
        }
        let piece = get_piece();
        let col = take_turn(board.get_width());
        let result: &str;
        if piece {
            result = board.place(Cell::TYellow, col-1);

        }
        else {
            result = board.place(Cell::ORed, col-1);
        }
        match result {
            "draw" => {
                println!("board filled with no winner: draw");
                break;
            },
            "full" => {
                println!("column is full, try again");
            },
            "toot" => {
                println!("toot wins");
                break;
            },
            "otto" => {
                println!("otto wins");
                break;
            },
            "tie" => {
                println!("result: tie");
                break;
            },
            _ => {
                // no issues, switch players for next turn
                if player {
                    player = false;
                }
                else {
                    player = true;
                }
            },
        }
    }
    // print the board one last time to show results
    print!("{}", board);

    true
}

fn main() {
    let mut finished = false;
    while !finished {
        println!("Which game would you like to play? (connect-4 or toot-otto)");
        let game = get_input();

        match game.as_str() {
            "connect-4" => { finished = start_c4(); },
            "toot-otto" => { finished = start_toototto(); },
            "q" | "-q" | "-quit" | "quit" | "exit" => { finished = true; },
            _ => { println!("not a valid option, try again"); },
        }
    }
}
