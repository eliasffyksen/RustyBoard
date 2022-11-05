use std::{fmt, io};

use super::super::players::InputParser;
use super::super::game;

type NaCAction = usize;

pub struct NaC {
    player_turn: usize,
    board: [usize; 9],
    terminal: bool,
    actions: Vec<NaCAction>
}

impl NaC {
    fn check_row(&self, row: usize) -> bool {
        self.board[row * 3] != 0
            && self.board[row * 3] == self.board[row * 3 + 1]
            && self.board[row * 3] == self.board[row * 3 + 2]
    }

    fn check_col(&self, col: usize) -> bool {
        self.board[col] != 0
            && self.board[col] == self.board[3 + col]
            && self.board[col] == self.board[6 + col]
    }

    pub fn to_string(&self) -> String {
        let mut str = String::new();
        for row in 0..3 {
            str.push_str("+---+---+---+\n|");

            for col in 0..3 {
                str.push(' ');

                let tile = row * 3 + col;
                str.push(match self.board[tile] {
                    1  => 'X',
                    2 => 'O',
                    _  => char::from_digit(
                        tile.try_into().unwrap(),
                        10
                    ).unwrap(),
                });
                str.push_str(" |");
            }
            str.push('\n');
        }
        str.push_str("+---+---+---+");
        str
    }


    pub fn new() -> NaC {
        NaC {
            player_turn: 0,
            board: [0; 9],
            terminal: false,
            actions: (0..9).collect()
        }
    }
}

impl InputParser<NaCAction> for NaC {
    fn parse_input(input: &String) -> io::Result<NaCAction> {
        let action = input.trim().parse::<NaCAction>();
        match action {
            Err(_) => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid user input"
            )),
            Ok(action) => Ok(action)
        }
    }
}

impl fmt::Display for NaC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl game::State<NaCAction> for NaC {

    fn get_actions(&self) -> Option<&Vec<NaCAction>> {
        if self.terminal {
            return None;
        }
        Some(&self.actions)
    }

    fn do_action(&self, action: usize) -> NaC {
        let mut new_state = NaC {
            player_turn: (self.player_turn + 1) % 2,
            board: self.board.clone(),
            terminal: false,
            actions: self.actions.clone()
        };

        let action = new_state.actions.remove(action);
        new_state.board[action] = self.player_turn + 1;

        if new_state.actions.len() == 0 {
            new_state.terminal = true;
            return new_state;
        }

        // Check terminal
        // Check row
        for row in 0..3 {
            if  new_state.check_row(row) {
                new_state.terminal = true;
                return new_state
            }
        }

        // Check col
        for col in 0..3 {
            if  new_state.check_col(col) {
                new_state.terminal = true;
                return new_state
            }
        }

        // Check diagonals
        if new_state.board[4] != 0 {
            // Check diagonal 1
            if new_state.board[4] == new_state.board[0]
                && new_state.board[4] == new_state.board[8] {
                new_state.terminal = true;
                return new_state;
            }

            // Check diagnoal 2
            if new_state.board[4] == new_state.board[2]
                && new_state.board[4] == new_state.board[6] {
                new_state.terminal = true;
                return new_state;
            }
        }

        new_state
    }

    fn get_player(&self) -> usize {
        return self.player_turn;
    }

    fn get_score(&self, player: usize) -> i32 {
        if !self.terminal {
            return 0;
        }

        if player == self.player_turn {
            0
        } else {
            1
        }
    }
}
