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
				Arg::new("match")
					.help(
						"filter suites by path glob, ie. `/test1.rs /e2e/`",
					)
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
				Arg::new("static")
				.required(false)
				.help("directory for static files (ie .css) that should be served")
				.long("static")
				.action(ArgAction::Set),
			)
			.arg(
				Arg::new("watch")
					.required(false)
					.help("do not watch the directory for changes")
					.short('w')
					.long("watch")
					.action(ArgAction::SetTrue),
				)
			.arg(
				Arg::new("interactive")
				.required(false)
				.help("don't run the tests, just start the server")
				.short('i')
				.long("interactive")
				.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("headed")
					.required(false)
					.help("run the tests using chromedriver in headed mode")
					.long("headed")
					.action(ArgAction::SetTrue),
			)
	}
	fn run(&self, args: &clap::ArgMatches) -> anyhow::Result<()> {
		let mut cli = SweetCli::default();
		cli.matches = args
			.get_many::<String>("match")
			.unwrap_or_default()
			.map(|s| s.clone())
			.collect::<Vec<_>>();


		cli.package = args.get_one::<String>("package").cloned();
		cli.static_dir = args.get_one::<String>("static").cloned();
		cli.server.address.secure = args.get_flag("secure");
		cli.release = args.get_flag("release");
		cli.watch = args.get_flag("watch");


		cli.run_tests_mode = if args.get_flag("interactive") {
			None
		} else if args.get_flag("headed") {
			Some(RunTestsMode::Headed)
		} else {
			Some(RunTestsMode::Headless)
		};

		cli.run()
	}
}
