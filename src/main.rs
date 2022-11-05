use std::io;

mod games;
mod game;
mod players;

fn main() {
    let state = games::NaC::new();

    let mut game = game::Game::new(state, vec![
        Box::new(players::InputPlayer::new(io::stdin().lock())),
        Box::new(players::RandomPlayer::new(rand::thread_rng())),
    ]);

    game.play();

    println!("{}", game.get_state());
}