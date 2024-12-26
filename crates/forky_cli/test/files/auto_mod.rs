use forky_cli::*;
use std::path::*;
use sweet::*;

#[sweet_test]
fn works() -> Result<()> {
    const EXPECTED: &str = r#"pub mod included_dir;
pub mod included_file;
#[allow(unused_imports)]
pub use self::included_file::*;
"#;

    let path = Path::new("crates/forky_cli/test/files/test_dir");

    let txt = auto_mod::create_mod_text(&path.to_path_buf());
    expect(txt.as_str()).to_be(EXPECTED)?;

    Ok(())
}
