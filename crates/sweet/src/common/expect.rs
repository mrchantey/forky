use crate::matchers::Matcher;
/// Alias for `Matcher::new()`
///
/// # Example
///
/// ```rust
/// expect(true).to_be_true()?;
/// expect("foobar").not().to_start_with("bar")?;
///
/// ```
pub fn expect<T>(value: T) -> Matcher<T> { Matcher::new(value) }
