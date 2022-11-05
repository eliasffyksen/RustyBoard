
use super::super::game;

pub struct RandomPlayer<R> where R: rand::Rng {
    rng: R
}

impl<R> RandomPlayer<R> where R: rand::Rng {
    pub fn new(rng: R) -> RandomPlayer<R> {
        RandomPlayer {
            rng: rng
        }
    }
}

impl<S, A, R> game::Player<S, A> for RandomPlayer<R>
where S: game::State<A>, R: rand::Rng {
    fn get_action(&mut self, s: &S) -> usize {
        let actions = s.get_actions()
            .expect("get_action called on terminal game");

        return self.rng.gen_range(0..actions.len());
    }
}
