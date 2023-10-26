use super::*;
use anyhow::Result;
use clap::Arg;
use clap::ArgAction;
use forky_core::OptionTExt;
use forky_fs::Subcommand;

pub struct ServerCommand;

impl Subcommand for ServerCommand {
	fn name(&self) -> &'static str { "serve" }
	fn about(&self) -> &'static str { "serve static files" }

	fn append_command(&self, command: clap::Command) -> clap::Command {
		command
			.arg(
				Arg::new("dir")
					.required(false)
					.default_value("./")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("port")
					.required(false)
					.help("specify port")
					.default_value("3000")
					.long("port")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("host")
					.required(false)
					.help("specify host")
					.default_value("0.0.0.0")
					.long("host")
					.action(ArgAction::Set),
			)
			.arg(
				Arg::new("secure")
					.required(false)
					.help("run with https")
					.long("secure")
					.action(ArgAction::SetTrue),
			)
			.arg(
				Arg::new("any-origin")
					.required(false)
					.help("add access-control-allow-origin: * header")
					.long("any-origin")
					.action(ArgAction::SetTrue),
			)
			// .arg(
			// 	Arg::new("proxies")
			// 		.required(false)
			// 		.help("add a proxy to serve from")
			// 		.long("proxy")
			// 		.action(ArgAction::Append),
			// )
			.arg(
				Arg::new("proxy")
					.required(false)
					.help("adds a proxy served from /_proxy/*")
					.long("proxy")
					.action(ArgAction::SetTrue),
			)
	}

	fn run(&self, args: &clap::ArgMatches) -> Result<()> {
		let dir = args.get_one::<String>("dir").ok()?;
		let port = args.get_one::<String>("port").ok()?;
		let host = args.get_one::<String>("host").ok()?;
		let secure = args.get_flag("secure");
		let any_origin = args.get_flag("any-origin");
		let proxy = args.get_flag("proxy");
		// let proxies = args
		// 	.get_many::<String>("proxies")
		// 	.unwrap()
		// 	.map(|s| s.clone())
		// 	.collect::<Vec<_>>();

		let server = Server {
			dir: dir.to_string(),
			any_origin,
			proxy,
			address: Address {
				host: Address::host_from_str(&host)?,
				port: port.parse::<u16>()?,
				secure,
				..Default::default()
			},
			..Default::default()
		};

		server.serve_with_default_reload()
		// server.with_dir(dir).serve_with_default_reload()
	}
}
