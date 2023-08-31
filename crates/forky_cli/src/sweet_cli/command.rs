use super::*;
use clap::Arg;
use clap::ArgAction;
use clap::Command;
use forky_fs::Subcommand;

pub struct SweetCommand;

const ABOUT: &str = "build, serve & run tests in-browser";

impl Subcommand for SweetCommand {
	fn name(&self) -> &'static str { "sweet" }
	fn about(&self) -> &'static str { ABOUT }
	fn append_command(&self, command: Command) -> Command {
		command
			.arg(
				Arg::new("match")
					.help(
						"filter suites by path glob, ie `my_test` or `/e2e/`",
					)
					.required(false)
					.action(ArgAction::Append),
			)
			.arg(
				Arg::new("example")
					.help("pass the --example flag to cargo run")
					.required(false)
					.short('e')
    			.default_value("test")
					.long("example"),
			)
			.arg(
				Arg::new("package")
					.help("pass the --package flag to cargo run")
					.required(false)
					.short('p')
					.long("package"),
			)
			.arg(
				Arg::new("cargo")
					.help("any additional args for cargo run")
					.required(false)
					.long("cargo"),
			)
			.arg(
				Arg::new("bindgen")
					.help("any additional args for wasm-bindgen")
					.required(false)
					.long("bindgen"),
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
				.help("directory for static files (ie .css) used by component tests")
				.long("static")
				.action(ArgAction::Set),
			)
			.arg(
				Arg::new("watch")
					.required(false)
					.help("live reload file changes")
					.short('w')
					.long("watch")
					.action(ArgAction::SetTrue),
				)
				.arg(
					Arg::new("headed")
					.required(false)
					.help("run the tests with a visible browser window")
					.long("headed")
					.action(ArgAction::SetTrue),
				)
				.arg(
					Arg::new("interactive")
					.required(false)
					.help("run the server continuously for viewing in your browser")
					.short('i')
					.long("interactive")
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

		cli.example = args.get_one::<String>("example").cloned().unwrap();
		cli.bindgen_args = args.get_one::<String>("bindgen").cloned();
		cli.cargo_args = args.get_one::<String>("cargo").cloned();
		cli.package = args.get_one::<String>("package").cloned();
		cli.static_dir = args.get_one::<String>("static").cloned();
		cli.server.address.secure = args.get_flag("secure");
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
