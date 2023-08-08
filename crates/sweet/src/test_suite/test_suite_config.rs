use crate::TestCaseConfig;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct TestSuiteConfig {
	#[serde(default)]
	pub cases: TestCaseConfig,
}
