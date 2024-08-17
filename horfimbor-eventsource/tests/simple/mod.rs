use horfimbor_eventsource_derive::{Command, Event, StateNamed};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use horfimbor_eventsource::{
    Command, CommandName, Dto, Event, EventName, State, StateName, StateNamed,
};

const SIMPLE_STATE_NAME: StateName = "SIMPLE_STATE_NAME";

#[derive(Deserialize, Serialize, Clone, Debug, Command)]
#[state(SIMPLE_STATE_NAME)]
pub enum SimpleCommand {
    Add(u32),
    Remove(u32),
    Set(u32),
}

#[derive(Error, Debug)]
pub enum SimpleError {
    #[error("the simple error is `{0}`")]
    Info(String),
}

#[derive(Deserialize, Serialize, Debug, Clone, Event)]
#[state(SIMPLE_STATE_NAME)]
pub enum SimpleEvent {
    Added(u32),
    Removed(u32),
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone, StateNamed)]
#[state(SIMPLE_STATE_NAME)]
pub struct SimpleState {
    pub nb: u32,
}

impl Dto for SimpleState {
    type Event = SimpleEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            SimpleEvent::Added(n) => self.nb += n,
            SimpleEvent::Removed(n) => self.nb -= n,
        }
    }
}

impl State for SimpleState {
    type Command = SimpleCommand;
    type Error = SimpleError;

    fn try_command(&self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            SimpleCommand::Add(n) => {
                if self.nb.checked_add(n).is_none() {
                    Err(SimpleError::Info(format!(
                        "{} cannot be added to {}",
                        n, self.nb
                    )))
                } else {
                    Ok(vec![SimpleEvent::Added(n)])
                }
            }
            SimpleCommand::Remove(n) => {
                if n > self.nb {
                    Err(SimpleError::Info(format!(
                        "{} cannot be removed to {}",
                        n, self.nb
                    )))
                } else {
                    Ok(vec![SimpleEvent::Removed(n)])
                }
            }
            SimpleCommand::Set(n) => Ok(vec![SimpleEvent::Removed(self.nb), SimpleEvent::Added(n)]),
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize, Clone)]
pub struct SimpleNbAddDto {
    pub nb: u32,
}

impl Dto for SimpleNbAddDto {
    type Event = SimpleEvent;

    fn play_event(&mut self, event: &Self::Event) {
        match event {
            SimpleEvent::Added(_) => self.nb += 1,
            _ => {}
        }
    }
}
