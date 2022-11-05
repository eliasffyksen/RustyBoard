use std::io;

mod games;
mod game;
mod players;

fn main() {
    let state = games::NaC::new();

    let mut game = game::Game::new(state, vec![
        Box::new(players::MinMax::new()),
        Box::new(players::InputPlayer::new(io::stdin().lock())),
    ]);

    game.play();

    println!("{}", game.get_state());
}