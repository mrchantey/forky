use super::*;
use anyhow::Result;
use clap::ArgMatches;
use forky_core::ArcMut;
use forky_fs::FsWatcher;

pub struct StyleAllCli {
	pub lightning: Option<Lightning>,
}

impl StyleAllCli {
	pub fn watch_with_mutex(&self, mutex: ArcMut<()>) -> Result<()> {
		Self::watcher().with_mutex(mutex).watch(|_| self.run())
	}

	pub fn watch(&self) -> Result<()> { Self::watcher().watch(|_| self.run()) }

	fn watcher() -> FsWatcher {
		FsWatcher::default()
			.with_watch("**/*.css")
			.with_ignore("**/index.css")
			.with_ignore("**/html/**")
			.with_ignore("**/target/**")
	}

	pub fn run(&self) -> Result<()> {
		create_type_files()?;
		create_index_files()?;

		if let Some(lightning) = &self.lightning {
			lightning.run()?;
		}

		Ok(())
	}
}

impl From<&ArgMatches> for StyleAllCli {
	fn from(value: &ArgMatches) -> Self {
		let lightning = value.get_one::<String>("lightning").cloned();
		let package = value.get_one::<String>("package").cloned();

		let lightning = match lightning {
			Some(dst) => {
				let src = if let Some(package) = package {
					format!("crates/{}/src/index.css", package)
				} else {
					"src/index.css".to_string()
				};
				Some(Lightning { src, dst })
			}
			None => None,
		};

		Self { lightning }
	}
}
