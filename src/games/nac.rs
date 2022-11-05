use std::{fmt, io};

use super::super::players::InputParser;
use super::super::game;

type NaCAction = usize;

pub struct NaC {
    player_turn: usize,
    board: [usize; 9],
    terminal: bool,
    draw: bool,
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
            draw: false,
            player_turn: 0,
            board: [0; 9],
            terminal: false,
            actions: (0..9).collect()
        }
    }

    fn set_terminal(&mut self) {
        if self.actions.len() == 0 {
            self.terminal = true;
            self.draw = true;
            return;
        }

        self.terminal = false;
        self.draw = false;

        // Check terminal
        // Check row
        for row in 0..3 {
            if  self.check_row(row) {
                self.terminal = true;
            }
        }

        // Check col
        for col in 0..3 {
            if  self.check_col(col) {
                self.terminal = true;
            }
        }

        // Check diagonals
        if self.board[4] != 0 {
            // Check diagonal 1
            if self.board[4] == self.board[0]
                && self.board[4] == self.board[8] {
                self.terminal = true;
            }

            // Check diagnoal 2
            if self.board[4] == self.board[2]
                && self.board[4] == self.board[6] {
                self.terminal = true;
            }
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
            draw: false,
            actions: self.actions.clone()
        };

        let action = new_state.actions.remove(action);
        new_state.board[action] = self.player_turn + 1;

        new_state.set_terminal();

        new_state
    }

    fn get_player(&self) -> usize {
        return self.player_turn;
    }

    fn get_score(&self, player: usize) -> i32 {
        if !self.terminal || self.draw {
            return 0;
        }

        if player == self.player_turn {
            -1
        } else {
            1
        }
    }
}
