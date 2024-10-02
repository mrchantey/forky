use anyhow::Result;
use enigo::Direction::*;
use enigo::*;
use std::time::Duration;

#[derive(Debug, Clone)]
pub enum Input {
    Key(Key),
    Combo(Vec<Key>),
    Text(String),
    Delay(Duration),
    Command(String),
}

impl Into<Input> for Key {
    fn into(self) -> Input {
        Input::Key(self)
    }
}

impl Into<Input> for Vec<Key> {
    fn into(self) -> Input {
        Input::Combo(self)
    }
}

impl Into<Input> for String {
    fn into(self) -> Input {
        Input::Text(self)
    }
}

impl Into<Input> for &str {
    fn into(self) -> Input {
        Input::Text(self.to_string())
    }
}

impl Into<Input> for Duration {
    fn into(self) -> Input {
        Input::Delay(self)
    }
}

impl Input {
    pub fn run(&self, enigo: &mut Enigo) -> Result<()> {
        match self {
            Input::Key(key) => {
                enigo.key(*key, Click)?;
            }
            Input::Combo(combo) => {
                let mut held: Vec<&Key> = combo.iter().collect();
                let last = held.pop().ok_or_else(|| anyhow::anyhow!("no key"))?;

                for key in held.iter() {
                    enigo.key(**key, Press)?;
                }
                enigo.key(*last, Click)?;
                for key in held.iter().rev() {
                    enigo.key(**key, Release)?;
                }
            }
            Input::Text(text) => {
                enigo.text(text)?;
            }
            Input::Delay(delay) => {
                std::thread::sleep(*delay);
            }
            Input::Command(command) => {
                // let mut args = command.split_whitespace();
                // let first = args.next().unwrap();
                let output = std::process::Command::new("sh")
                    // std::process::Command::new(first)
                    .arg("-c")
                    .args(command.split_whitespace())
                    // .args(args)
                    .output()?;
                println!("{:?}", output);
                output.stdout.iter().for_each(|&c| print!("{}", c as char));
                output.stderr.iter().for_each(|&c| eprint!("{}", c as char));
            }
        }
        Ok(())
    }
}
