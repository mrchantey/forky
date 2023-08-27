# Macros

Currently there is only one macro for use: `sweet!()`. This will create a suite to be collected by the runner.

```rs
sweet!{
	it "works"{}
	test "is an alias for it"{}
	it skip "wont run"{}
	it only "will exclude non-onlys in this suite"{}
	it e2e "(in-browser) runs in the parent process"{}
}
```
