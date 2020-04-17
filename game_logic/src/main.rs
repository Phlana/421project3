use std::io;
use crate::board_mod::Cell;
use crate::board_mod::board_mod::BoardLike;
use crate::connect4::C4Board;
use crate::toototto::TOBoard;

mod board_mod;
mod connect4;
mod toototto;

extern crate rand;
use rand::Rng;

fn get_input() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("failed to read line");
    string = string.trim().to_lowercase();

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

fn start_c4() {
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
}

// true for T, false for O
fn get_piece() -> bool {
    loop {
        println!("Which piece do you want to place? (T or O)");
        let piece = get_input();

        match piece.as_str() {
            "t" => { return true; },
            "o" => { return false; },
            _ => { println!("not a valid piece, try again"); },
        }
    }
}

fn start_toototto() {
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
}

fn computer_turn<T: BoardLike>(board: T, width: usize) -> usize {
    // find a valid not full column to place
    let mut not_found = false;
    let mut col: usize = 0;
    let mut rng = rand::thread_rng();
    while !not_found {
        col = rng.gen_range(0, width);

        not_found = board.check_column(col);
    }
    col + 1
}

fn computer_c4() {
    let mut board = C4Board::new(7, 6);

    // true for yellow turn, false for red (bot) turn
    let mut player = true;

    loop {
        print!("{}", board);
        let col: usize;
        if player {
            println!("It is yellow's turn");
            col = take_turn(board.get_width());
        }
        else {
            println!("It is red's turn");
            col = computer_turn(board.clone(), board.get_width());
        }
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
}

fn computer_piece() -> bool {
    let mut rng = rand::thread_rng();
    return rng.gen();
}

fn computer_toototto() {
    let mut board = TOBoard::new(6, 4);

    let mut player = true;

    loop {
        print!("{}", board);
        let piece: bool;
        let col: usize;
        if player {
            println!("It is toot's turn");
            piece = get_piece();
            col = take_turn(board.get_width());
        }
        else {
            println!("It is otto's turn");
            piece = computer_piece();
            col = computer_turn(board.clone(), board.get_width());
        }

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
}

fn main() {
    // true for connect 4, false for toot otto
    let mode: bool;
    // true for human opponent, false for computer opponent
    let versus: bool;

    loop {
        println!("Which game would you like to play? (connect-4 or toot-otto)");
        let game = get_input();

        match game.as_str() {
            "connect-4" => {
                mode = true;
                break;
            },
            "toot-otto" => {
                mode = false;
                break;
            },
            "q" | "-q" | "-quit" | "quit" | "exit" => { return; },
            _ => { println!("not a valid option, try again"); },
        }
    }

    loop {
        println!("Would you like to play against a person or computer? (person or computer)");
        let opponent = get_input();

        match opponent.as_str() {
            "person" => {
                versus = true;
                break;
            },
            "computer" => {
                versus = false;
                break;
            },
            "q" | "-q" | "-quit" | "quit" | "exit" => { return; },
            _ => { println!("not a valid option, try again"); },
        }
    }

    if mode && versus {
        start_c4();
    }
    else if mode {
        computer_c4();
    }
    else if versus {
        start_toototto();
    }
    else {
        computer_toototto();
    }
}
