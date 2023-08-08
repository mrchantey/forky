use crate::TestSuiteContext;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TestCaseConfig {
	#[serde(default)]
	pub skip: bool,
	#[serde(default)]
	pub only: bool,
	#[serde(default)]
	pub context: Option<TestSuiteContext>,
}
