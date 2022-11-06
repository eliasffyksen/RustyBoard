
#[derive(Copy, Clone)]
enum Piece {
  Pawn,
  Knight,
  Rook,
  Bishop,
  Queen,
  King(bool), // Whether it has moved or not
}

use std::fmt;

use Piece::*;

#[derive(Copy, Clone)]
enum Square {
  Blank,
  White(Piece),
  Black(Piece),
}

use Square::*;

pub struct State {
  player_turn: u8,
  check: bool,
  mate: bool,
  board: [Square; 64],
}


impl State {
  pub fn new() -> State {
    let mut state = State {
      player_turn: 0,
      check: false,
      mate: false,
      board: [Blank; 64],
    };
    state.init_board();
    state
  }

  fn init_board(&mut self) {

    // Init pawns
    for i in 0..8 {
      self.init_piece(Pawn, 8 + i);
    }

    self.init_piece(Rook, 0);
    self.init_piece(Rook, 7);

    self.init_piece(Knight, 1);
    self.init_piece(Knight, 6);

    self.init_piece(Bishop, 2);
    self.init_piece(Bishop, 5);

    self.init_piece(Queen, 3);
    self.init_piece(King(false), 3);
  }

  fn init_piece(&mut self, piece: Piece, place: usize) {
    self.board[place] = Black(piece);
    self.board[63 - place] = White(piece);
  }
}
