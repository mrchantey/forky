use super::*;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use forky_fs::Subcommand;

pub struct SweetCommand;

const ABOUT: &str = "build the wasm sweet runner and start a dev server";

impl Subcommand for SweetCommand {
	fn name(&self) -> &'static str { "sweet" }
	fn about(&self) -> &'static str { ABOUT }
	fn append_command(&self, command: Command) -> Command {
		command
			.arg(
				Arg::new("specify-suite")
					.required(false)
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("package")
					.help("pass the --package flag to cargo run")
					.required(false)
					.short('p')
					.long("package"),
			)
			.arg(
				Arg::new("release")
					.required(false)
					.help("pass the --release flag to cargo run")
					.long("release")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("secure")
					.required(false)
					.help("run the dev server with https")
					.long("secure")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("nowatch")
					.required(false)
					.help("do not watch the directory for changes")
					.long("nowatch")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("static")
					.required(false)
					.help("directory for static files (ie .css) that should be served")
					.long("static")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("run")
					.required(false)
					.help("run the tests using chromedriver")
					.long("run")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("run-headless")
					.required(false)
					.help("run the tests using chromedriver in headless mode")
					.long("run-headless")
					.action(ArgAction::Set),
			)
	}
	fn run(&self, args: &clap::ArgMatches) -> anyhow::Result<()> {
		let mut cli = SweetCli::default();
		cli.package = args.get_one::<String>("package").cloned();
		cli.static_dir = args.get_one::<String>("static").cloned();
		cli.server.address.secure = args.get_flag("secure");
		cli.release = args.get_flag("release");
		cli.watch = !args.get_flag("nowatch");


		
		cli.run()
	}
}
