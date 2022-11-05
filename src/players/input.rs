use std::{io::{self, BufRead, Write}, fmt};

use super::super::game;

pub trait InputParser<A> {
    fn parse_input(input: &String) -> io::Result<A>;
}

pub struct InputPlayer<B>
where B: BufRead {
    reader: B
}

impl<B> InputPlayer<B>
where B: BufRead {
    pub fn new(reader: B)->InputPlayer<B> {
        InputPlayer{ reader }
    }
}

impl<S, A, B> game::Player<S, A> for InputPlayer<B>
where
    S: game::State<A>,
    S: fmt::Display,
    Vec<A>: fmt::Debug,
    S: InputParser<A>,
    B: BufRead,
    A: std::cmp::PartialEq
{

    fn get_action(&mut self, state: &S) -> usize {
        let actions = state.get_actions().expect("No actions available");
        println!("{state}");
        println!("Please select an action: {:?}", actions);
        let mut buf = String::new();

        loop {
            buf.clear();
            print!("> ");
            io::stdout().flush().expect("Error flushing output");

            match self.reader.read_line(&mut buf) {
                Ok(s) if s > 0 => (),
                _ => panic!("Error while reading input"),
            }

            match S::parse_input(&buf) {
                Ok(action) => {
                    let mut result = None;
                    for i in 0..actions.len() {
                        if actions[i] == action {
                            result = Some(i);
                            break;
                        }
                    }

                    match result {
                        Some(action) => break action,
                        None => {
                            println!("Action not available");
                        }
                    }
                },
                Err(err) => {
                    println!("Error: {err}")
                }
            }
        }
    }
}
