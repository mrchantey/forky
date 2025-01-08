#![cfg(not(target_arch = "wasm32"))]
use ::forky_cli::prelude::*;

fn main() -> anyhow::Result<()> { ForkyCli::run() }
