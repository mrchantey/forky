use forky_core::prelude::*;
use std::path::Path;
use std::path::PathBuf;

pub struct CliPathExt;

impl CliPathExt {
	pub fn push_with(
		parent: impl AsRef<Path>,
		path: impl AsRef<Path>,
	) -> PathBuf {
		let mut child = parent.as_ref().to_path_buf();
		child.push(path);
		child
	}
	pub fn filename_starts_with_uppercase(path: impl AsRef<Path>) -> bool {
		path.as_ref().file_name().str().first().is_ascii_uppercase()
	}

	pub fn anscestors_includes(
		path: impl AsRef<Path>,
		arr: impl IntoIterator<Item = impl AsRef<Path>>,
	) -> bool {
		arr.into_iter().any(|f| {
			path.as_ref()
				.ancestors()
				.any(|p| p.file_stem().unwrap_or_default() == f.as_ref())
		})
	}

	pub fn filename_included(
		path: impl AsRef<Path>,
		arr: impl IntoIterator<Item = impl AsRef<Path>>,
	) -> bool {
		arr.into_iter().any(|f| {
			path.as_ref().file_stem().unwrap_or_default() == f.as_ref()
		})
	}
	pub fn filestem_starts_with_underscore(path: impl AsRef<Path>) -> bool {
		path.as_ref().file_stem().str().first() == '_'
	}
	pub fn filestem_ends_with_underscore(path: impl AsRef<Path>) -> bool {
		path.as_ref().file_stem().str().last() == '_'
	}
	pub fn filestem_ends_with_double_underscore(
		path: impl AsRef<Path>,
	) -> bool {
		path.as_ref().file_stem().str().ends_with("__")
	}
	pub fn filestem_contains_double_underscore(path: impl AsRef<Path>) -> bool {
		path.as_ref().file_stem().str().contains("__")
	}

	pub fn pop_first_two_path_components(path: &str) -> PathBuf {
		let mut components = Path::new(path).components();
		components.next();
		components.next();
		components.as_path().to_path_buf()
	}
}
