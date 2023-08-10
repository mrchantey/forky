# Matchers


Matchers are an ergonomic way to make assertions. Providing intellisense and type-specific assertions, they can make for an enjoyable testing experience.

The `expect(val)` function returns a `Matcher<T>` where `T` is the type of the value passed in. What assertions are available for that matcher depend on the 

```rs
```

```rs
expect(true).to_be_false()?;
```
<img src="images/failure.png" width=50%>

## Negation

All matchers can be negated by calling `not()`

```rs
expect("foobar").not().to_contain("bazz")?;
```

## Built-in Matchers

Some examples of built-in matchers are:

- String
	```rs
	expect("foobar").to_start_with("foo")?;
	```
- Result
  ```rs
	expect(my_result).to_be_ok()?;
	```
- Numbers (ord)
	```rs
	expect(2).to_be_greater_than(1)?;
	```


## Extending Matchers

Matchers are easy to extend, particulary using the `extend` crate.

```rust
# use anyhow::Result;
# use extend::ext;
# use sweet::*;
# 
# #[derive(Debug)]
struct Awesomeness(u32);

#[ext(name)]
pub impl Matcher<Awesomeness> {
	fn to_be_more_awesome_than(&self, other:Awesomeness) -> Result<()> {
		let outcome = self.0 > other.0;
		let expected = format!("to be more awesome than {:?}", other);
		self.assert_correct(outcome, &expected)
	}
}
```

Note that here we are calling `self.assert_correct()` which does to things:
- checks the outcome is true, or false in the case of negation:
	- `expect(foo).not().to_be_more_awesome_than(bar)`
- Formats a pretty backtraced output error if needed.
