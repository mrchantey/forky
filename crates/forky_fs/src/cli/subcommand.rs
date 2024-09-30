use anyhow::anyhow;
use anyhow::Result;
use clap::ArgMatches;
use clap::Command;
use std::io;

/// A helper for splitting [`clap::Command`] into subcommands, allowing
/// for modular composition.
pub trait Subcommand {
	fn name(&self) -> &'static str;
	fn about(&self) -> &'static str;
	fn version(&self) -> &'static str { "0.0.1" }
	/// Add your arguments here
	fn append_command(&self, command: Command) -> Command { command }

	/// Usually only used internally Internal use only
	fn create_command(&self) -> Command {
		let mut cmd = Command::new(self.name())
			.about(self.about())
			.version(self.version());

		let subs = self.subcommands();
		if subs.len() > 0 {
			cmd = cmd
				// .subcommand_required(true)
				// .arg_required_else_help(true)
				.allow_external_subcommands(true);
		}
		for sub in subs.iter() {
			cmd = cmd.subcommand(sub.create_command());
		}
		cmd = self.append_command(cmd);
		cmd
	}

	fn subcommands(&self) -> Vec<Box<dyn Subcommand>> { vec![] }

	fn run_with_str(&self, args: String) -> Result<()> {
		let args = self
			.create_command()
			.get_matches_from(args.split_whitespace());
		self.run_subcommand_or_default(&args)
	}

	fn run_with_cli_args(&self) -> Result<()> {
		let args = self.create_command().get_matches();
		self.run_subcommand_or_default(&args)
	}
	fn run_subcommand_or_default(&self, args: &ArgMatches) -> Result<()> {
		let mut sub_match = false;
		for (name, args) in args.subcommand().iter() {
			for sub in self.subcommands().iter() {
				if sub.name() == *name {
					sub_match = true;
					sub.run_subcommand_or_default(args)?;
				}
			}
		}
		if !sub_match {
			match self.run(args) {
				Ok(_) => {}
				Err(e) => {
					if e.to_string()
						== "No default function or subcommand entered.."
					{
						self.create_command().print_help()?;
					}
					return Err(e);
				}
			}
		}
		Ok(())
	}

	fn run(&self, _args: &ArgMatches) -> Result<()> {
		// Err(unimplemented!("No default function or subcommand entered.."))
		Err(anyhow!("No default function or subcommand entered.."))
	}


	/// Run in a repl loop, reading from stdin.
	fn run_repl(&self) -> Result<()> {
		let stdin = io::stdin(); // We get `Stdin` here.
		loop {
			let mut input = String::new();
			stdin.read_line(&mut input)?;
			self.run_with_str(input)?;
		}
	}
}
