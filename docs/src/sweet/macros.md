# Macros

# `#[sweet_test]`

Tests can be declared via an attribute.

```rs
#[sweet_test]
fn foobar(){}

//accepts several flags, async functions or an `anyhow::Result` return type
#[sweet_test(skip,only,e2e,non_send)]
async fn foobar()->Result<()>{
	expect(true).to_be_true()
}
```

# `sweet!`

A layout more familiar to front-end developers. Note that rust formatters may not indent etc. the contents of this macro correctly.

```rs
sweet!{
	it "has less boilerplate" {
		expect(true).to_be_true()?;
	}
	test "is an alias for it"{}
	it skip "wont run"{}
	it only "will exclude non-onlys in this suite"{}
	it e2e "(in-browser) runs in the parent process"{}
}
```
