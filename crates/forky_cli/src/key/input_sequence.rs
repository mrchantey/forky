use super::*;
use anyhow::Result;
use enigo::{Enigo, Key, Settings};
use std::time::Duration;

#[derive(Debug)]
pub struct InputSequence {
    commands: Vec<Input>,
    delay: Duration,
}

impl Default for InputSequence {
    fn default() -> Self {
        Self {
            delay: Duration::from_millis(500),
            commands: Vec::new(),
        }
    }
}

impl InputSequence {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn input(mut self, command: impl Into<Input>) -> Self {
        self.commands.push(command.into());
        self
    }
    pub fn command(mut self, command: impl Into<String>) -> Self {
        self.commands.push(Input::Command(command.into()));
        self
    }

    pub fn ulaunch(self, workspace: usize, search: &str) -> Self {
        self.input(vec![
            Key::Meta,
            Key::Unicode(workspace.to_string().chars().next().unwrap()),
        ])
        .input(vec![Key::Alt, Key::Space])
        .input(search)
        .input(Key::UpArrow) // ensure its the last result, exact match
        .input(Key::Return)
        .input(Duration::from_secs(1))
    }

    pub fn run(&mut self) -> Result<()> {
        self.run_with_enigo(&mut Enigo::new(&Settings::default())?)
    }
    pub fn run_with_enigo(&mut self, enigo: &mut Enigo) -> Result<()> {
        for command in self.commands.iter() {
            command.run(enigo)?;
            std::thread::sleep(self.delay);
        }
        Ok(())
    }
}
