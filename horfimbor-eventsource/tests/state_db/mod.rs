use horfimbor_eventsource_derive::{Command, Event, StateNamed};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use horfimbor_eventsource::*;

const POKE_STATE_NAME: StateName = "POKE_STATE_NAME";

#[derive(Deserialize, Serialize, Clone, Debug, Command)]
#[state(POKE_STATE_NAME)]
pub enum PokeCommand {
    Poke(u32),
}

#[derive(Error, Debug)]
pub enum PokeError {
    #[error("the Poke error is `{0}`")]
    Info(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, Event)]
#[state(POKE_STATE_NAME)]
pub enum PokeEvent {
    Poked(u32),
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone, StateNamed)]
#[state(POKE_STATE_NAME)]
pub struct PokeState {
    pub nb: u32,
}

impl Dto for PokeState {
    type Event = PokeEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            PokeEvent::Poked(n) => self.nb += n,
        }
    }
}

impl State for PokeState {
    type Command = PokeCommand;
    type Error = PokeError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            PokeCommand::Poke(n) => {
                if self.nb.checked_add(n).is_none() {
                    Err(PokeError::Info(format!(
                        "{} cannot be added to {}",
                        n, self.nb
                    )))
                } else {
                    Ok(vec![PokeEvent::Poked(n)])
                }
            }
        }
    }
}
