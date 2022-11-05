pub trait State<A> {
    fn get_actions(&self) -> Option<&Vec<A>>;
    fn do_action(&self, action: usize) -> Self;
    fn get_player(&self) -> usize;
    fn get_score(&self, player: usize) -> i32;
}

pub trait Player<S, A> where S: State<A> {
    fn get_action(&mut self, s: &S) -> usize;
}

pub struct Game<S, A>
where S: State<A> {
    state: S,
    players: Vec<Box<dyn Player<S, A>>>
}

impl<S, A> Game<S, A> where S: State<A> {
    pub fn new(state: S, players: Vec<Box<dyn Player<S, A>>>) -> Game<S, A> {
        Game { state, players }
    }

    pub fn round(&mut self) {
        let player = self.state.get_player();
        let action = self.players[player].get_action(&self.state);
        self.state = self.state.do_action(action);
    }

    pub fn play(&mut self) {
        while let Some(_) = self.state.get_actions() {
            self.round()
        }
    }

    pub fn get_state(&self) -> &S {
      return &self.state;
    }
}
