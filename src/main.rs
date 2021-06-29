#[path = "./mod/clear_console.rs"]
mod console;

#[path = "./mod/board.rs"]
mod board;

#[path = "./mod/input_value.rs"]
mod input;

#[path = "./mod/loading.rs"]
mod loading;

#[path = "./mod/check_winner.rs"]
mod winner;

#[derive(Debug)]
enum Players {
    X,
    O,
}

#[derive(Debug)]
struct TicTacToe {
    matrix: [&'static str; 9],
    current_player: Players,
}

fn check_value_exists(value: &'static str) -> bool {
    match value {
        "X" => true,
        "O" => true,
        _ => false,
    }
}

fn declare_result(result: i8) {
    match result {
        1 => println!("\nPlayer X is the winner!"),
        0 => println!("\nMatch drawn!"),
        -1 => println!("\nPlayer O is the winner!"),
        _ => (),
    }
}

fn main() {
    let mut tic_tac_toe = TicTacToe {
        matrix: ["1", "2", "3", "4", "5", "6", "7", "8", "9"],
        current_player: Players::X,
    };

    let mut _turns: u8 = 0;

    loop {
        console::clear_console();
        println!("---- Tic Tac Toe Game ----\n\n");
        board::print_board(&tic_tac_toe.matrix);

        if winner::check_winner(&tic_tac_toe.matrix) {
            match tic_tac_toe.current_player {
                Players::X => declare_result(-1),
                Players::O => declare_result(1),
            }

            break;
        }
        
        if _turns == 9 {
            declare_result(0);
            break;
        }

        // Gets position/index in which the player wishes to place
        let position = match tic_tac_toe.current_player {
            Players::X => input::input_value(&String::from("\n[Player X]\nEnter position: "))
                .trim()
                .parse::<usize>()
                .expect("Wrong number format!"),
            Players::O => input::input_value(&String::from("\n[Player O]\nEnter position: "))
                .trim()
                .parse::<usize>()
                .expect("Wrong number format!"),
        };

        if position > 10 {
            println!("The index ranges from 1 to 9, please try again!");
            loading::loading_animation();
            console::clear_console();
        } else if check_value_exists(&tic_tac_toe.matrix[position - 1]) {
            println!("Index already contains a value, please try again!");
            loading::loading_animation();
            console::clear_console();
        } else {
            match tic_tac_toe.current_player {
                Players::X => {
                    tic_tac_toe.matrix[position - 1] = "X";
                    tic_tac_toe.current_player = Players::O;
                }
                Players::O => {
                    tic_tac_toe.matrix[position - 1] = "O";
                    tic_tac_toe.current_player = Players::X;
                }
            };

            _turns += 1;
        }
    }
}
