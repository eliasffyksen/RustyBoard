use std::fmt;

use super::super::game;

pub struct MinMax {}

impl MinMax {
  pub fn new() -> MinMax {
    MinMax {}
  }
}

impl<S, A> game::Player<S, A> for MinMax
where S: game::State<A>, S: fmt::Display {
    fn get_action(&mut self, state: &S) -> usize {

      let mut best_score = None;
      let mut best_action = 0;
      let actions = state.get_actions().expect("get action called on terminal state");

      for i in 0..actions.len() {
        let new_state = get_best_state(state.do_action(i));
        let score = new_state.get_score(state.get_player());

        match best_score {
          Some(best_score) if best_score > score => (),
          _ => {
            best_score = Some(score);
            best_action = i;
          }
        }
      }
      best_action
    }
}

fn get_best_state<S,A>(state: S) -> S
where S: game::State<A> {
  match state.get_actions() {
    None => state,
    Some(actions) => {
      let mut best_score = None;
      let mut best_state = None;

      for i in 0..actions.len() {
        let new_state = get_best_state(state.do_action(i));
        let score = new_state.get_score(state.get_player());

        match best_score {
            Some(best_score) if best_score > score => (),
            _ => {
              best_score = Some(score);
              best_state = Some(new_state);
            }
        }
      }

      best_state.expect("No valid action found")
    }
  }
}