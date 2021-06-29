#[path = "./mod/clear_console.rs"]
mod console;

#[path = "./mod/options.rs"]
mod options;

#[path = "./mod/board.rs"]
mod board;

#[path = "./mod/input_value.rs"]
mod input;

#[path = "./mod/loading.rs"]
mod loading;

#[path = "./mod/rand.rs"]
mod rand_num_gen;

#[path = "./mod/check_winner.rs"]
mod winner;

enum Players {
    X,
    O,
}

enum Options {
    Singleplayer,
    Multiplayer,
    Exit,
}

struct TicTacToe {
    matrix: [&'static str; 9],
    current_player: Players,
    game_type: Options,
}

impl TicTacToe {
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

    fn intro() -> Options {
        console::clear_console();
        println!("---- Tic Tac Toe Game ----\n\n");

        match options::type_of_play() {
            1 => Options::Singleplayer,
            2 => Options::Multiplayer,
            _ => Options::Exit,
        }
    }

    fn get_position_in_number(player: &'static str) -> usize {
        // Gets position/index in which the player wishes to place
        input::input_value(&String::from(format!(
            "\n[Player {}]\nEnter position: ",
            player
        )))
        .trim()
        .parse::<usize>()
        .expect("Wrong number format!")
    }

    fn game(&mut self) {
        let mut _turns: u8 = 0;

        loop {
            console::clear_console();
            println!("---- Tic Tac Toe Game ----\n\n");
            board::print_board(&self.matrix);

            if winner::check_winner(&self.matrix) {
                match &self.current_player {
                    Players::X => TicTacToe::declare_result(-1),
                    Players::O => TicTacToe::declare_result(1),
                }

                break;
            }

            if _turns == 9 {
                TicTacToe::declare_result(0);
                break;
            }

            let position = match self.game_type {
                Options::Singleplayer => match &self.current_player {
                    Players::X => TicTacToe::get_position_in_number("X"),
                    Players::O => rand_num_gen::generate_random_position(1, 9),
                },
                Options::Multiplayer => match &self.current_player {
                    Players::X => TicTacToe::get_position_in_number("X"),
                    Players::O => TicTacToe::get_position_in_number("O"),
                },
                _ => 0,
            };

            if position > 10 {
                println!("The index ranges from 1 to 9, please try again!");
                loading::loading_animation();
                console::clear_console();
            } else if TicTacToe::check_value_exists(&self.matrix[position - 1]) {
                println!("Index already contains a value, please try again!");
                loading::loading_animation();
                console::clear_console();
            } else {
                match &self.current_player {
                    Players::X => {
                        self.matrix[position - 1] = "X";
                        self.current_player = Players::O;
                    }
                    Players::O => {
                        self.matrix[position - 1] = "O";
                        self.current_player = Players::X;
                    }
                };

                _turns += 1;
            }
        }
    }
}

fn main() {
    let option = TicTacToe::intro();

    let mut tic_tac_toe = TicTacToe {
        matrix: ["1", "2", "3", "4", "5", "6", "7", "8", "9"],
        current_player: Players::X,
        game_type: option,
    };

    match tic_tac_toe.game_type {
        Options::Singleplayer | Options::Multiplayer => TicTacToe::game(&mut tic_tac_toe),
        Options::Exit => println!("Exiting...!"),
    }
}
